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
  const [tens_color, units_color] = bands
  return 10 * colorCode(tens_color) + colorCode(units_color)
}
