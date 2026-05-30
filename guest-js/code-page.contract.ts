import {
  ENCODE,
  type CodePage,
  type PrintJobRequest,
  text,
} from './index'

const configuredCodePage: CodePage = {
  code_page: 6,
  encode: ENCODE.ACCENT_REMOVER,
  use_gbk: false,
}

const request: PrintJobRequest = {
  printer: 'Printer',
  paper_size: 'Mm58',
  options: configuredCodePage,
  sections: [text('hola')],
}

void request
