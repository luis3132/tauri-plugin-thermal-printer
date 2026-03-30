import { invoke } from '@tauri-apps/api/core'

export type PaperSize =
  | 'Mm40'   // 40mm — ~21 chars/line
  | 'Mm44'   // 44mm — ~24 chars/line
  | 'Mm58'   // 58mm — ~32 chars/line (small format)
  | 'Mm72'   // 72mm — ~42 chars/line
  | 'Mm80'   // 80mm — ~48 chars/line (standard large format)
  | 'Mm104'  // 104mm — ~62 chars/line (wide format)

export const PAPER_SIZE_CHARS_PER_LINE: Record<PaperSize, number> = {
  Mm40: 21,
  Mm44: 24,
  Mm58: 32,
  Mm72: 42,
  Mm80: 48,
  Mm104: 62,
}

export const PAPER_SIZE_PIXELS_WIDTH: Record<PaperSize, number> = {
  Mm40: 256,
  Mm44: 288,
  Mm58: 384,
  Mm72: 512,
  Mm80: 576,
  Mm104: 752,
}

export const DEFAULT_PAPER_SIZE: PaperSize = 'Mm80'

export function getPaperSizeCharsPerLine(paperSize: PaperSize): number {
  return PAPER_SIZE_CHARS_PER_LINE[paperSize]
}

export function getPaperSizePixelsWidth(paperSize: PaperSize): number {
  return PAPER_SIZE_PIXELS_WIDTH[paperSize]
}

export interface PrinterOptions {
  cut_paper: boolean
  beep: boolean
  open_cash_drawer: boolean
}

export interface GlobalStyles {
  bold?: boolean
  underline?: boolean
  align?: 'left' | 'center' | 'right'
  italic?: boolean
  invert?: boolean
  font?: 'A' | 'B' | 'C'
  rotate?: boolean
  upside_down?: boolean
  size?: 'normal' | 'height' | 'width' | 'double'
}

export interface Title {
  text: string
  styles?: GlobalStyles
}

export interface Subtitle {
  text: string
  styles?: GlobalStyles
}

export interface Text {
  text: string
  styles?: GlobalStyles
}

export interface Feed {
  feed_type: 'lines' | 'dots' | 'line_feed'
  value: number
}

export interface Cut {
  mode: 'full' | 'partial' | 'partial_alt' | 'partial_alt2'
  feed: number
}

export interface Beep {
  times: number
  duration: number
}

export interface Drawer {
  pin: 2 | 5
  pulse_time: number
}

export interface Table {
  columns: number
  column_widths?: number[]
  header?: Text[]
  body: Text[][]
  truncate: boolean
}

export interface Qr {
  data: string
  size: number
  error_correction: 'L' | 'M' | 'Q' | 'H'
  model: 1 | 2
  align?: 'left' | 'center' | 'right'
}

export interface Barcode {
  data: string
  barcode_type: 'UPC-A' | 'UPC-E' | 'EAN13' | 'EAN8' | 'CODE39' | 'ITF' | 'CODABAR' | 'CODE93' | 'CODE128'
  width: number
  height: number
  text_position: 'none' | 'above' | 'below' | 'both'
  align?: 'left' | 'center' | 'right'
}

export interface DataMatrixModel {
  data: string
  size: number
}

export interface Pdf417 {
  data: string
  columns: number
  rows: number
  width: number
  height: number
  error_correction: number
}

export interface Image {
  data: string
  max_width: number
  align: 'left' | 'center' | 'right'
  dithering: boolean
  size: 'normal' | 'double_width' | 'double_height' | 'quadruple'
}

export interface Logo {
  key_code: number
  mode: 'normal' | 'double_width' | 'double_height' | 'quadruple'
}

export interface Line {
  character: string
}

export type PrintSections =
  | { Title: Title }
  | { Subtitle: Subtitle }
  | { Text: Text }
  | { Feed: Feed }
  | { Cut: Cut }
  | { Beep: Beep }
  | { Drawer: Drawer }
  | { GlobalStyles: GlobalStyles }
  | { Qr: Qr }
  | { Barcode: Barcode }
  | { Table: Table }
  | { DataMatrix: DataMatrixModel }
  | { Pdf417: Pdf417 }
  | { Image: Image }
  | { Logo: Logo }
  | { Line: Line }

export interface PrintJobRequest {
  printer: string
  sections: PrintSections[]
  options: PrinterOptions
  paper_size: PaperSize
}

export interface PrinterInfo {
  name: string
  interface_type: string
  identifier: string
  status: string
}

export interface TestPrintRequest {
  printer_info: PrintJobRequest
  include_text?: boolean
  include_text_styles?: boolean
  include_alignment?: boolean
  include_columns?: boolean
  include_separators?: boolean
  include_barcode?: boolean
  include_barcode_types?: boolean
  include_qr?: boolean
  include_image?: boolean
  image_base64?: string | null
  include_beep?: boolean
  test_cash_drawer?: boolean
  cut_paper?: boolean
  test_feed?: boolean
  test_all_fonts?: boolean
  test_invert?: boolean
  test_rotate?: boolean
}

/**
 * Sends a print job to the specified thermal printer.
 * @throws {string} Error message from the printer or document generation if the job fails.
 */
export async function print_thermal_printer(printJobRequest: PrintJobRequest): Promise<void> {
  await invoke('plugin:thermal-printer|print_thermal_printer', {
    printJobRequest,
  })
}

/**
 * Returns the list of available thermal printers on the current system.
 * @throws {string} Error message if printer enumeration fails.
 */
export async function list_thermal_printers(): Promise<PrinterInfo[]> {
  return await invoke<PrinterInfo[]>('plugin:thermal-printer|list_thermal_printers')
}

/**
 * Sends a test print job to verify the printer is working correctly.
 * @throws {string} Error message from the printer or document generation if the job fails.
 */
export async function test_thermal_printer(testPrintRequest: TestPrintRequest): Promise<void> {
  await invoke('plugin:thermal-printer|test_thermal_printer', {
    printTestRequest: testPrintRequest,
  })
}
