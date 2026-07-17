import Foundation

/// Discovers network printers advertised over Bonjour (mDNS/DNS-SD) without scanning the
/// subnet. Mirrors the Android NSD discovery. Prefers `_pdl-datastream._tcp` (RAW/JetDirect,
/// port 9100 — the service that understands ESC/POS over a raw socket).
///
/// > Note: the host app's `Info.plist` must declare `NSLocalNetworkUsageDescription` and list
/// > the service types under `NSBonjourServices`, otherwise iOS 14+ blocks discovery.
final class NetworkDiscovery: NSObject, NetServiceBrowserDelegate, NetServiceDelegate {

  private var browsers: [NetServiceBrowser] = []
  private var pendingServices: [NetService] = []
  private var results: [String: [String: Any]] = [:] // host -> PrinterInfo
  private var completion: (([[String: Any]]) -> Void)?
  private var finished = false

  private let serviceTypes = [
    "_pdl-datastream._tcp.", // RAW / JetDirect (port 9100) — ESC/POS
    "_printer._tcp.",        // LPD
    "_ipp._tcp."             // IPP
  ]

  func discover(timeout: TimeInterval = 4.0, completion: @escaping ([[String: Any]]) -> Void) {
    self.completion = completion

    // NetServiceBrowser is run-loop based — drive it on the main run loop and never block it.
    DispatchQueue.main.async {
      for type in self.serviceTypes {
        let browser = NetServiceBrowser()
        browser.delegate = self
        browser.searchForServices(ofType: type, inDomain: "local.")
        self.browsers.append(browser)
      }
    }

    DispatchQueue.main.asyncAfter(deadline: .now() + timeout) { [weak self] in
      self?.finish()
    }
  }

  func netServiceBrowser(_ browser: NetServiceBrowser, didFind service: NetService, moreComing: Bool) {
    service.delegate = self
    pendingServices.append(service) // keep a strong reference while resolving
    service.resolve(withTimeout: 3.0)
  }

  func netServiceDidResolveAddress(_ sender: NetService) {
    guard let addresses = sender.addresses else { return }
    for addressData in addresses {
      guard let ip = Self.ipv4String(from: addressData) else { continue }
      let isRaw = sender.type.contains("pdl-datastream")
      let port = (isRaw && sender.port > 0) ? sender.port : Self.rawPrintPort

      let existing = results[ip]
      // Prefer the RAW (9100) entry over LPD/IPP for the same host.
      if existing == nil || (isRaw && !(existing?["identifier"] as? String ?? "").hasSuffix(":\(Self.rawPrintPort)")) {
        results[ip] = [
          "name": sender.name.isEmpty ? "Network Printer" : sender.name,
          "interface_type": "Network",
          "identifier": "\(ip):\(port)",
          "status": "Available"
        ]
      }
      break
    }
  }

  func netService(_ sender: NetService, didNotResolve errorDict: [String: NSNumber]) {
    // Ignore — unresolved services are simply skipped.
  }

  private func finish() {
    if finished { return }
    finished = true
    for browser in browsers { browser.stop() }
    completion?(Array(results.values))
    completion = nil
  }

  private static let rawPrintPort = 9100

  /// Extracts a dotted IPv4 string from a `sockaddr` blob returned by `NetService.addresses`.
  private static func ipv4String(from data: Data) -> String? {
    return data.withUnsafeBytes { (raw: UnsafeRawBufferPointer) -> String? in
      guard let base = raw.baseAddress else { return nil }
      let sa = base.assumingMemoryBound(to: sockaddr.self)
      guard sa.pointee.sa_family == sa_family_t(AF_INET) else { return nil }
      var addr = base.assumingMemoryBound(to: sockaddr_in.self).pointee.sin_addr
      var buffer = [CChar](repeating: 0, count: Int(INET_ADDRSTRLEN))
      inet_ntop(AF_INET, &addr, &buffer, socklen_t(INET_ADDRSTRLEN))
      return String(cString: buffer)
    }
  }
}
