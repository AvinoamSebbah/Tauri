export const pallete = {
  transparent: 'transparent',
  current: 'currentColor',
  white: '#ffffff',
  black: {
    DEFAULT: '#000000',
  },

  primary: {
    25: '#F0F6FE',
    50: '#E7F0FE',
    100: '#DDEAFE',
    150: '#CEE2FD',
    200: '#BAD6FC',
    250: '#9DC4FB',
    300: '#80B3FA',
    350: '#62A1F8',
    400: '#3B89F7',
    500: '#0A6CF5',
    600: '#0856C4',
    700: '#064193',
    800: '#002666',
    900: '#001333',
  },

  neutral: {
    25: '#F6F7F9',
    50: '#F0F2F5',
    100: '#E9EDF1',
    150: '#DDE2E9',
    200: '#D2D8DF',
    250: '#C6CDD7',
    300: '#BAC3CE',
    350: '#A8B3C2',
    400: '#9AA6B7',
    500: '#7C8CA2',
    600: '#626E89',
    700: '#4C566B',
    800: '#3B3F4A',
    900: '#24272D',
  },

  red: {
    25: '#FFFBFA',
    50: '#FEF3F2',
    100: '#FEE4E2',
    200: '#FED9D7',
    300: '#FDB9B4',
    400: '#F97066',
    500: '#F04438',
    600: '#D92D20',
    700: '#B42318',
    800: '#912018',
    900: '#7A271A',
  },

  green: {
    25: '#F6FEF9',
    50: '#ECFDF3',
    100: '#D1FADF',
    200: '#A6F4C5',
    300: '#6CE9A6',
    400: '#32D583',
    500: '#12B76A',
    600: '#039855',
    700: '#027A48',
    800: '#05603A',
    900: '#054F31',
  },

  orange: {
    25: '#FFFCF5',
    50: '#FFFAEB',
    100: '#FEF4D7',
    150: '#FEEEBE',
    200: '#FEDE86',
    300: '#FDC958',
    400: '#FDB022',
    500: '#F79009',
    600: '#DC6803',
    700: '#B54708',
    800: '#93370D',
    900: '#792E0D',
  },

  purple: {
    25: '#F8F5FF',
    50: '#F2EBFF',
    100: '#E4D7FE',
    150: '#D3BEFE',
    200: '#AE86FE',
    300: '#8F58FD',
    400: '#6B22FD',
    500: '#5809F7',
    600: '#4B03DC',
    700: '#4208B5',
    800: '#3A0D93',
    900: '#310D79',
  },

  lime: {
    25: '#FAFCE3',
    50: '#F7F9DC',
    100: '#EBF391',
    150: '#E5F075',
    200: '#DFEC51',
    300: '#D6E723',
    400: '#CCDC18',
    500: '#BBCA16',
    600: '#919C11',
    700: '#808A0F',
    800: '#666E0C',
    900: '#4C5309',
  },

  offwhite: {
    25: '#FCFCFD',
  },
}

export const colors = {
  ...pallete,

  base: {
    black: pallete.neutral[800],
    white: pallete.white,
    neutral: pallete.offwhite[25],
  },
}

export const textColors = {
  ...colors,
  primary: colors.base.black,
  inverted: colors.base.white,
  secondary: pallete.neutral[600],
  third: pallete.neutral[400],
  light: pallete.neutral[150],
  placeholder: pallete.neutral[400],
  disabled: pallete.neutral[250],
  error: pallete.red[600],
  success: pallete.green[600],
  'on-action': pallete.primary[500],
  active: pallete.green[500],
  cancelled: pallete.red[500],
  end: pallete.lime[600],
  hold: pallete.orange[500],
}

export const bgColors = {
  ...colors,

  primary: {
    ...pallete.primary,
    DEFAULT: pallete.primary[500], // buttons
    hover: pallete.primary[600], // primary default on hover
    press: pallete.primary[700], // primary default on press
    'level-1': pallete.primary[25], // secondary buttons bg
    'level-2': pallete.primary[50], // elements on level 1 / level 1 hover
    'level-3': pallete.primary[100], // elements on level 2 / level 2 hover
    'level-4': pallete.primary[150], // elements on level 3 / level 3 hover
    'level-5': pallete.primary[200], // elements on level 4 / level 4 hover
  },

  neutral: {
    ...pallete.neutral,
    'level-1': pallete.neutral[25], // elements on level 2
    'level-2': pallete.neutral[50], // elements on level 2
    'level-3': pallete.neutral[100], // elements on level 2
    'level-4': pallete.neutral[150], // secondary buttons bg
    'level-5': pallete.neutral[200], // secondary buttons bg
    'level-8': pallete.neutral[500], // secondary buttons bg
  },

  error: {
    ...pallete.red,
    DEFAULT: pallete.red[500], // fill color for error
    hover: pallete.red[600], // error default on hover
    press: pallete.red[700], // error default on press
    'level-1': pallete.red[100], // secondary buttons bg
  },

  success: {
    ...pallete.green,
    DEFAULT: pallete.green[600], // fill color for success
    hover: pallete.green[700], // success default on hover
    press: pallete.green[800], // success default on press
    'level-1': pallete.green[50], // secondary buttons bg
  },

  status: {
    end: pallete.lime[50], //  bg, Active going to End car status
    active: pallete.green[50], //   bg, Active car status
    cancelled: pallete.red[50], //   bg, Cancelled car status
    hold: pallete.orange[50], //   bg, Hold car status
  },

  table: {
    status: {
      green: pallete.green[400], // status sign
      red: pallete.red[400], // status sign
      orange: pallete.orange[300], // status sign
      grey: pallete.neutral[200], // status sign
      purple: pallete.purple[400], // status sign
      black: pallete.neutral[800], // status sign
    },
    highlight: {
      red: pallete.red[100], // highlight color
      'red-hover': pallete.red[200], // highlight color on hover
      orange: pallete.orange[100], // highlight color
      'orange-hover': pallete.orange[150], // highlight color on hover
    },
  },
}

export const borderColors = {
  ...colors,

  DEFAULT: pallete.neutral[800],
  black: pallete.neutral[800],
  dark: pallete.neutral[600],
  medium: pallete.neutral[350],
  light: pallete.neutral[150],
  'extra-light': pallete.neutral[100],

  primary: {
    ...colors.primary,
    ...bgColors.primary,
    DEFAULT: pallete.primary[400],
    dark: pallete.primary[250],
  },

  neutral: {
    ...bgColors.neutral,
  },

  success: {
    DEFAULT: pallete.green[600],
    dark: pallete.green[500],
    light: pallete.green[400],
  },

  warning: {
    DEFAULT: pallete.orange[600],
    dark: pallete.orange[500],
  },

  error: {
    DEFAULT: pallete.red[600],
    dark: pallete.red[500],
  },

  table: {
    ...bgColors.table,
  },
}
