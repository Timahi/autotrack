import * as jspdf from 'jspdf'
import { save } from '@tauri-apps/plugin-dialog'
import { create } from '@tauri-apps/plugin-fs'

export default defineNuxtPlugin(() => {
  const { $vehicleService, $maintenanceService } = useNuxtApp()

  const addLineBreaks = (str: string, lineLength = 45) => {
    return str.replace(new RegExp(`(.{${lineLength}})`, 'g'), '$1\n')
  }

  const countLines = (str: string) => {
    const lines = str.split('\n')
    return lines[lines.length - 1] === '' ? lines.length - 1 : lines.length
  }

  const maintenanceBookletService = {
    async generate(vehicleId: number) {
      return new Promise<void>(async (resolve, reject) => {
        try {
          const vehicle = await $vehicleService.getById(vehicleId)
          const maintenance = await $maintenanceService.getAll(vehicleId)

          // noinspection JSPotentiallyInvalidConstructorUsage
          const doc = new jspdf.jsPDF()

          doc.setFontSize(16)
          doc.text(
            `${vehicle.brand} ${vehicle.model} - ${vehicle.registration} : ${formatDate(new Date())}`,
            30,
            20
          )

          let yIndex = 40

          // TODO: implement new pages

          for (const m of maintenance.sort(
            (a, b) => b.performedAt.getTime() - a.performedAt.getTime()
          )) {
            doc.setFontSize(14)
            doc.text(`${formatDate(m.performedAt)} - ${m.odometer}km - ${m.type}`, 20, yIndex)

            if (m.description) {
              yIndex += 5
              doc.setFontSize(12)

              const description = addLineBreaks(m.description)

              doc.text(description, 25, yIndex)

              yIndex += countLines(description) * 5 + 5
            } else {
              yIndex += 10
            }
          }

          const path = await save({
            filters: [
              {
                name: 'PDF',
                extensions: ['pdf'],
              },
            ],
          })

          if (!path) {
            // noinspection ExceptionCaughtLocallyJS
            throw 'CANCELED'
          }

          const file = await create(path)
          const output = doc.output('arraybuffer')
          await file.write(new Uint8Array(output))
          await file.close()

          resolve()
        } catch (error) {
          reject("Échec lors de la génération du carnet d'entretien")
        }
      })
    },
  }

  return {
    provide: {
      maintenanceBookletService,
    },
  }
})
