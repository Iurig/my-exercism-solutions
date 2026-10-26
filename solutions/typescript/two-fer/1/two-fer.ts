/**
 * This stub is provided to make it straightforward to get started.
 */

export function twoFer(name = 'you'): string {
  return `One for ${name}, one for me.`
  // ^                 ^   ^ this is called a return type; it's the type of the
  // ^                 ^     value that is returned from this function
  // ^                 ^
  // ^                 parameters go here
  // ^
  // allows the tests to import this function and call it
  // <-- Your code goes here. You may remove all the commentary in this file.
}
