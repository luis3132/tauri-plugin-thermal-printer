import logoTienda from '../assets/images/logo-tienda.png'
import degradadoEsfera from '../assets/images/degradado-esfera.png'
import calibracion from '../assets/images/calibracion-80mm.png'
import productoCaja from '../assets/images/producto-caja.png'

/** Imágenes incluidas en la app para probar sin depender de archivos externos. */
export const SAMPLE_IMAGES = [
  { id: 'logo', name: 'Logo de tienda', url: logoTienda, hint: 'Blanco y negro puro, 384 px' },
  { id: 'esfera', name: 'Degradado / esfera', url: degradadoEsfera, hint: 'Grises: prueba el dithering' },
  { id: 'calibracion', name: 'Regla 80 mm', url: calibracion, hint: '576 px: comprueba el ancho útil' },
  { id: 'caja', name: 'Icono de producto', url: productoCaja, hint: 'Pequeña, 200 px' },
]

function blobToDataUrl(blob) {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.onload = () => resolve(reader.result)
    reader.onerror = () => reject(reader.error)
    reader.readAsDataURL(blob)
  })
}

/** Devuelve la imagen como data URI base64 (el plugin acepta el prefijo `data:`). */
export async function urlToDataUrl(url) {
  const res = await fetch(url)
  return blobToDataUrl(await res.blob())
}

export const fileToDataUrl = blobToDataUrl

/** Base64 sin el prefijo `data:...;base64,`. */
export function stripDataUrl(dataUrl) {
  return dataUrl.slice(dataUrl.indexOf(',') + 1)
}

function loadImage(src) {
  return new Promise((resolve, reject) => {
    const img = new Image()
    img.onload = () => resolve(img)
    img.onerror = () => reject(new Error('No se pudo cargar la imagen'))
    img.src = src
  })
}

/**
 * Simula en un canvas cómo quedará la imagen en papel térmico: escala al ancho
 * máximo, pasa a gris y aplica Floyd–Steinberg o umbral, como hace el plugin.
 */
export async function renderThermalPreview(canvas, src, { maxWidth, dithering, paperWidth }) {
  const img = await loadImage(src)
  const limit = maxWidth > 0 ? Math.min(maxWidth, paperWidth) : paperWidth
  const scale = img.width > limit ? limit / img.width : 1
  const w = Math.max(1, Math.round(img.width * scale))
  const h = Math.max(1, Math.round(img.height * scale))

  canvas.width = w
  canvas.height = h
  const ctx = canvas.getContext('2d', { willReadFrequently: true })
  ctx.fillStyle = '#fff'
  ctx.fillRect(0, 0, w, h)
  ctx.drawImage(img, 0, 0, w, h)

  const data = ctx.getImageData(0, 0, w, h)
  const px = data.data
  const gray = new Float32Array(w * h)
  for (let i = 0; i < w * h; i++) {
    const a = px[i * 4 + 3] / 255
    const g = 0.299 * px[i * 4] + 0.587 * px[i * 4 + 1] + 0.114 * px[i * 4 + 2]
    gray[i] = g * a + 255 * (1 - a)
  }

  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      const i = y * w + x
      const v = gray[i] < 128 ? 0 : 255
      if (dithering) {
        const err = gray[i] - v
        if (x + 1 < w) gray[i + 1] += (err * 7) / 16
        if (y + 1 < h) {
          if (x > 0) gray[i + w - 1] += (err * 3) / 16
          gray[i + w] += (err * 5) / 16
          if (x + 1 < w) gray[i + w + 1] += err / 16
        }
      }
      px[i * 4] = px[i * 4 + 1] = px[i * 4 + 2] = v
      px[i * 4 + 3] = 255
    }
  }
  ctx.putImageData(data, 0, 0)
  return { width: w, height: h, original: { width: img.width, height: img.height } }
}
