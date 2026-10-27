export const COLORS = [
  'black',
  'brown',
  'red',
  'orange',
  'yellow',
  'green',
  'blue',
  'violet',
  'grey',
  'white',
] as const

type Color = (typeof COLORS)[number]

export const colorCode = (color: Color): number => COLORS.indexOf(color)

export function decodedValue(bands: Color[]): number {
  const [tensColor, unitsColor] = bands
  return 10 * colorCode(tensColor) + colorCode(unitsColor)
}
