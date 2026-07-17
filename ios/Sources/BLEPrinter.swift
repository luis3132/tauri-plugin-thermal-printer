import Foundation
import CoreBluetooth

/// Sends a raw ESC/POS payload to a BLE printer via CoreBluetooth.
///
/// The `identifier` must be `"BLE:<peripheral-uuid>"`, as produced by `BLEDiscovery`
/// (iOS exposes a per-device UUID, never the hardware MAC address).
///
/// > Note: the host app's `Info.plist` must declare `NSBluetoothAlwaysUsageDescription`.
final class BLEPrinter: NSObject, CBCentralManagerDelegate, CBPeripheralDelegate {

  private let queue = DispatchQueue(label: "com.luis3132.thermal_printer.ble.print")

  private var central: CBCentralManager?
  private var targetUUID: UUID?
  private var peripheral: CBPeripheral?
  private var writeCharacteristic: CBCharacteristic?
  private var writeType: CBCharacteristicWriteType = .withoutResponse

  private var payload = Data()
  private var chunks: [Data] = []
  private var completion: ((Error?) -> Void)?
  private var finished = false

  func printRawData(identifier: String, data: Data, completion: @escaping (Error?) -> Void) {
    let uuidString = String(identifier.dropFirst("BLE:".count))
    guard let uuid = UUID(uuidString: uuidString) else {
      completion(Self.error("Invalid BLE identifier: \(identifier)"))
      return
    }

    queue.async {
      self.targetUUID = uuid
      self.payload = data
      self.completion = completion
      self.central = CBCentralManager(delegate: self, queue: self.queue)

      self.queue.asyncAfter(deadline: .now() + Self.timeoutSeconds) { [weak self] in
        self?.finish(Self.error("BLE operation timeout"))
      }
    }
  }

  private func finish(_ error: Error?) {
    if finished { return }
    finished = true
    if let peripheral = peripheral {
      central?.cancelPeripheralConnection(peripheral)
    }
    central?.stopScan()
    completion?(error)
    completion = nil
  }

  // MARK: - CBCentralManagerDelegate

  func centralManagerDidUpdateState(_ central: CBCentralManager) {
    switch central.state {
    case .poweredOn:
      guard let uuid = targetUUID else { return }
      if let known = central.retrievePeripherals(withIdentifiers: [uuid]).first {
        peripheral = known
        known.delegate = self
        central.connect(known, options: nil)
      } else {
        central.scanForPeripherals(withServices: nil, options: nil)
      }
    case .unauthorized:
      finish(Self.error("Bluetooth permission not granted"))
    case .poweredOff:
      finish(Self.error("Bluetooth is turned off"))
    case .unsupported:
      finish(Self.error("BLE is not supported on this device"))
    default:
      break
    }
  }

  func centralManager(
    _ central: CBCentralManager,
    didDiscover peripheral: CBPeripheral,
    advertisementData: [String: Any],
    rssi RSSI: NSNumber
  ) {
    guard peripheral.identifier == targetUUID else { return }
    central.stopScan()
    self.peripheral = peripheral
    peripheral.delegate = self
    central.connect(peripheral, options: nil)
  }

  func centralManager(_ central: CBCentralManager, didConnect peripheral: CBPeripheral) {
    peripheral.discoverServices(nil)
  }

  func centralManager(
    _ central: CBCentralManager,
    didFailToConnect peripheral: CBPeripheral,
    error: Error?
  ) {
    finish(error ?? Self.error("Failed to connect to BLE printer"))
  }

  // MARK: - CBPeripheralDelegate

  func peripheral(_ peripheral: CBPeripheral, didDiscoverServices error: Error?) {
    if let error = error { finish(error); return }
    for service in peripheral.services ?? [] {
      peripheral.discoverCharacteristics(nil, for: service)
    }
  }

  func peripheral(
    _ peripheral: CBPeripheral,
    didDiscoverCharacteristicsFor service: CBService,
    error: Error?
  ) {
    if writeCharacteristic != nil { return }
    if let error = error { finish(error); return }

    for characteristic in service.characteristics ?? [] {
      if characteristic.properties.contains(.writeWithoutResponse) {
        writeCharacteristic = characteristic
        writeType = .withoutResponse
        break
      } else if characteristic.properties.contains(.write) {
        writeCharacteristic = characteristic
        writeType = .withResponse
      }
    }

    if let characteristic = writeCharacteristic {
      startWriting(peripheral, characteristic)
    }
  }

  func peripheral(
    _ peripheral: CBPeripheral,
    didWriteValueFor characteristic: CBCharacteristic,
    error: Error?
  ) {
    if let error = error { finish(error); return }
    writeNext(peripheral, characteristic)
  }

  // MARK: - Writing

  private func startWriting(_ peripheral: CBPeripheral, _ characteristic: CBCharacteristic) {
    let mtu = peripheral.maximumWriteValueLength(for: writeType)
    let chunkSize = max(20, min(mtu, 180))

    chunks = []
    var offset = 0
    while offset < payload.count {
      let end = min(offset + chunkSize, payload.count)
      chunks.append(payload.subdata(in: offset..<end))
      offset = end
    }

    writeNext(peripheral, characteristic)
  }

  private func writeNext(_ peripheral: CBPeripheral, _ characteristic: CBCharacteristic) {
    guard !chunks.isEmpty else {
      // Allow the last packets to flush before tearing down the connection.
      queue.asyncAfter(deadline: .now() + 0.3) { [weak self] in self?.finish(nil) }
      return
    }

    let chunk = chunks.removeFirst()
    peripheral.writeValue(chunk, for: characteristic, type: writeType)

    // For writeWithoutResponse there is no ACK callback — pace the stream slightly.
    if writeType == .withoutResponse {
      queue.asyncAfter(deadline: .now() + 0.02) { [weak self] in
        self?.writeNext(peripheral, characteristic)
      }
    }
  }

  private static let timeoutSeconds: TimeInterval = 20.0

  private static func error(_ message: String) -> NSError {
    NSError(domain: "BLEPrinter", code: -1, userInfo: [NSLocalizedDescriptionKey: message])
  }
}
