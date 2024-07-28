import type { Config } from 'tailwindcss'


export default <Partial<Config>>{
  theme: {
    extend: {
      lineHeight: {
        '12': '4rem',
      },
      listStyleType: {
        'square': 'square',
      }
    }
  }
}
