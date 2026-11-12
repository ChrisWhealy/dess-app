import init, { start } from './pkg/dess_app.js'

const banner = document.getElementById('page-error')
try {
  await init()
  start()
} catch (error) {
  console.error(error)
  banner.textContent = `The simulator could not start: ${error?.message ?? error}`
  banner.hidden = false
}
