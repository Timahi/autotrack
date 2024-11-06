export const { format: formatDate } = Intl.DateTimeFormat('fr-FR', {
  day: '2-digit',
  month: '2-digit',
  year: 'numeric',
  timeZone: 'Europe/Paris',
})

export const { format: formatFullDate } = Intl.DateTimeFormat('fr-FR', {
  day: 'numeric',
  month: 'short',
  year: 'numeric',
  timeZone: 'Europe/Paris',
})

export const { format: formatTime } = Intl.DateTimeFormat('fr-FR', {
  hour: '2-digit',
  minute: '2-digit',
  timeZone: 'Europe/Paris',
})

export const { format: formatNumber } = Intl.NumberFormat('fr-FR')

export const { format: format2DigitsNumber } = Intl.NumberFormat('fr-FR', {
  minimumIntegerDigits: 2,
  useGrouping: false,
})
