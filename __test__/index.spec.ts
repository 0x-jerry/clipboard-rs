import { readText, readFiles, readImage, writeText } from '..'

describe('clipboard-rs', () => {
  it('write 123 clipboard, then read 123 from clipboard', () => {
    expect(writeText('123')).toBe(true)

    expect(readText()).toBe('123')
  })
})
