import Foundation
import Network

/// Sends a raw ESC/POS payload to a network printer over a RAW/JetDirect TCP socket
/// (typically port 9100). The `identifier` must be `"<host>:<port>"`, as produced by
/// `NetworkDiscovery`.
final class NetworkPrinter {

  private var connection: NWConnection?
  private var finished = false
  private let lock = NSLock()

  func printRawData(identifier: String, data: Data, completion: @escaping (Error?) -> Void) {
    guard let sep = identifier.lastIndex(of: ":") else {
      completion(Self.error("Invalid network identifier: \(identifier)"))
      return
    }
    let hostPart = String(identifier[identifier.startIndex..<sep])
    let portPart = String(identifier[identifier.index(after: sep)...])

    guard let portNumber = UInt16(portPart), let port = NWEndpoint.Port(rawValue: portNumber) else {
      completion(Self.error("Invalid network port: \(identifier)"))
      return
    }

    let connection = NWConnection(host: NWEndpoint.Host(hostPart), port: port, using: .tcp)
    self.connection = connection

    let finish: (Error?) -> Void = { [weak self] error in
      guard let self = self else { return }
      self.lock.lock()
      if self.finished { self.lock.unlock(); return }
      self.finished = true
      self.lock.unlock()
      connection.cancel()
      completion(error)
    }

    connection.stateUpdateHandler = { state in
      switch state {
      case .ready:
        connection.send(content: data, completion: .contentProcessed { sendError in
          finish(sendError)
        })
      case .failed(let error):
        finish(error)
      case .cancelled:
        break
      default:
        break
      }
    }

    // Guard against a connection that never becomes ready.
    DispatchQueue.global().asyncAfter(deadline: .now() + Self.timeoutSeconds) {
      finish(Self.error("Network connection timeout"))
    }

    connection.start(queue: .global())
  }

  private static let timeoutSeconds: TimeInterval = 10.0

  private static func error(_ message: String) -> NSError {
    NSError(domain: "NetworkPrinter", code: -1, userInfo: [NSLocalizedDescriptionKey: message])
  }
}
