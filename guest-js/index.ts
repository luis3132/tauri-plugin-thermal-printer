import { invoke } from '@tauri-apps/api/core'

export type PaperSize = 'Mm58' | 'Mm80'

export interface PrinterOptions {
  cut_paper: boolean
  beep: boolean
  open_cash_drawer: boolean
}

export interface GlobalStyles {
  bold?: boolean
  underline?: boolean
  align?: 'left' | 'center' | 'right' | string
  italic?: boolean
  invert?: boolean
  font?: 'A' | 'B' | 'C' | 'D' | 'E' | string
  rotate?: boolean
  upside_down?: boolean
  size?: 'normal' | 'height' | 'width' | 'double' | string
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
  feed_type: 'lines' | 'pixels' | string
  value: number
}

export interface Cut {
  mode: 'full' | 'partial' | string
  feed: number
}

export interface Beep {
  times: number
  duration: number
}

export interface Drawer {
  pin: number
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
  error_correction: 'L' | 'M' | 'Q' | 'H' | string
  model: number
  align?: 'left' | 'center' | 'right' | string
}

export interface Barcode {
  data: string
  barcode_type: 'UPCA' | 'UPCE' | 'EAN13' | 'EAN8' | 'CODE39' | 'ITF' | 'CODABAR' | 'CODE93' | 'CODE128' | string
  width: number
  height: number
  text_position: 'none' | 'above' | 'below' | 'both' | string
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
  align: 'left' | 'center' | 'right' | string
  dithering: boolean
  size: 'normal' | 'double_width' | 'double_height' | 'double' | string
}

export interface Logo {
  key_code: number
  mode: 'normal' | 'double_width' | 'double_height' | 'double' | string
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

export async function print_thermal_printer(printJobRequest: PrintJobRequest): Promise<boolean> {
  return await invoke('plugin:thermal-printer|print_thermal_printer', {
    printJobRequest: printJobRequest,
  })
}

export async function list_thermal_printers(): Promise<PrinterInfo[]> {
  return await invoke<PrinterInfo[]>('plugin:thermal-printer|list_thermal_printers')
}

export async function test_thermal_printer(testPrintRequest: TestPrintRequest): Promise<boolean> {
  return await invoke('plugin:thermal-printer|test_thermal_printer', {
    printTestRequest: testPrintRequest,
  })
}
