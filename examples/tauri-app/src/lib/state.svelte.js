import {
  list_thermal_printers,
  print_thermal_printer,
  test_thermal_printer,
  getPaperSizeCharsPerLine,
  getPaperSizePixelsWidth,
  ENCODE,
} from 'tauri-plugin-thermal-printer'

// En escritorio el plugin imprime por NOMBRE (lp -d / spooler de Windows);
// en Android/iOS enruta por IDENTIFIER (MAC, VID:/PID:, host:puerto).
export const isMobile = /android|iphone|ipad|ipod/i.test(navigator.userAgent)

export const PAPER_SIZES = ['Mm40', 'Mm44', 'Mm58', 'Mm72', 'Mm80', 'Mm104']

export const ENCODINGS = Object.keys(ENCODE)

export const app = $state({
  printers: [],
  /** Valor que se envía en `printer` (nombre o identifier según la plataforma). */
  target: '',
  /** Destino escrito a mano (p. ej. una IP:9100 o una MAC no listada). */
  manualTarget: '',
  useManual: false,
  paperSize: 'Mm58',
  codePage: 0,
  encode: 'ACCENT_REMOVER',
  useGbk: false,
  busy: false,
  logs: [],
})

export function printerTarget(p) {
  return isMobile ? p.identifier : p.name
}

export function currentTarget() {
  return app.useManual ? app.manualTarget.trim() : app.target
}

export function charsPerLine() {
  return getPaperSizeCharsPerLine(app.paperSize)
}

export function pixelsWidth() {
  return getPaperSizePixelsWidth(app.paperSize)
}

export function log(level, message) {
  app.logs.unshift({ time: new Date().toLocaleTimeString(), level, message })
  if (app.logs.length > 200) app.logs.length = 200
}

export function buildRequest(sections) {
  return {
    printer: currentTarget(),
    sections,
    options: { code_page: Number(app.codePage), encode: app.encode, use_gbk: app.useGbk },
    paper_size: app.paperSize,
  }
}

export async function refreshPrinters() {
  app.busy = true
  try {
    const printers = await list_thermal_printers()
    app.printers = printers
    log('info', `Encontradas ${printers.length} impresora(s).`)
    const targets = printers.map(printerTarget)
    if (!targets.includes(app.target)) app.target = targets[0] ?? ''
  } catch (e) {
    log('error', `Error al listar impresoras: ${e}`)
  } finally {
    app.busy = false
  }
}

async function run(label, fn) {
  if (!currentTarget()) {
    log('error', 'Selecciona una impresora (o escribe un destino manual) primero.')
    return false
  }
  app.busy = true
  const started = performance.now()
  try {
    await fn()
    log('ok', `${label} → enviado a "${currentTarget()}" (${Math.round(performance.now() - started)} ms)`)
    return true
  } catch (e) {
    log('error', `${label} → ${e}`)
    return false
  } finally {
    app.busy = false
  }
}

export function printSections(label, sections) {
  return run(label, () => print_thermal_printer(buildRequest(sections)))
}

export function printTest(label, flags) {
  return run(label, () => test_thermal_printer({ printer_info: buildRequest([]), ...flags }))
}
