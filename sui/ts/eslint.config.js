import antfu from '@antfu/eslint-config';

export default antfu({
  markdown: false,
  ignores: ['./src/_codegen/*'],
  rules: {
    'no-console': 'warn',
    'curly': ['error', 'all'],
    'camelcase': 'off',
    'no-useless-constructor': 'error',
    'no-control-regex': 'error',
    'antfu/top-level-function': 'off',
    'node/prefer-global/process': ['error', 'always'],
    'test/no-import-node-test': 'off',
    'eslint-comments/no-unlimited-disable': 'off',

    'style/member-delimiter-style': 'error',
    'style/semi': ['error', 'always'],
    'style/quotes': ['error', 'single'],
    'style/indent': ['error', 2],
    'style/arrow-parens': ['error', 'always'],
    'style/brace-style': ['error', '1tbs'],
    'style/comma-dangle': ['error', 'always-multiline'],
    'style/object-property-newline': ['error', { allowAllPropertiesOnSameLine: true }],
    'style/space-before-function-paren': [
      'error',
      {
        anonymous: 'never',
        named: 'never',
        asyncArrow: 'always',
      },
    ],

    'ts/no-unused-vars': [
      'error',
      {
        argsIgnorePattern: '^_',
        destructuredArrayIgnorePattern: '^_',
        ignoreRestSiblings: true,
      },
    ],
    // 'ts/consistent-type-imports': 'error',
    'perfectionist/sort-exports': 'off',
    'ts/array-type': [
      'error',
      {
        default: 'array',
      },
    ],
  },
});
