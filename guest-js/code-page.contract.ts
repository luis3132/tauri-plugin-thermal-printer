import {
  ENCODE,
  type CodePage,
  type PrintJobRequest,
  text,
} from './index'

const configuredCodePage: CodePage = {
  codepage: 6,
  encode: ENCODE.ACCENT_REMOVER,
  use_gbk: false,
}

const request: PrintJobRequest = {
  printer: 'Printer',
  paper_size: 'Mm58',
  options: {
    cut_paper: false,
    beep: false,
    open_cash_drawer: false,
    code_page: configuredCodePage,
  },
  sections: [text('hola')],
}

void request
