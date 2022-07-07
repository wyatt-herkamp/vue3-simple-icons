const path = require('path')
const writtenNumber = require('written-number');
const fs = require('fs-extra')
const simple = require('simple-icons/icons')
const {pascalCase} = require("pascal-case");
const componentTemplate = (name, svg) => `
<template>
    ${svg.replace(/<svg([^>]+)>/, '<svg>')}
</template>
<script>

export default {
  name: '${name}',

  props: {
    size: {
      type: String,
      default: '24',
      validator: (s) => (!isNaN(s) || s.length >= 2 && !isNaN(s.slice(0, s.length -1)) && s.slice(-1) === 'x' )
    }
  },

  functional: true,

  setup(props ,ctx) {
    const size = props.size.slice(-1) === 'x'
      ? props.size.slice(0, props.size.length -1) + 'em'
      : parseInt(props.size) + 'px';

    return {}
  }
}
</script>
`.trim()

function handleNumbers(title){
  const beginningNumbers = title.replace( /[^\d].*/, '' )
  if(beginningNumbers.length){
    const numberInEnglish = writtenNumber(beginningNumbers, {noAnd: true})
    const numberSlug = numberInEnglish.toString().split(' ').join('-') + '-'
    title = title.replace(beginningNumbers, numberSlug)
  }
  title = title.split('+').join('Plus')
  title = title.normalize("NFD").replace(/[\u0300-\u036f]/g, "")
  return title
}
const icons = Object.entries(simple).map(([key, value]) => ({

  key,
  pascalCasedComponentName: pascalCase(`${handleNumbers(key.substring(2, key.length))}-icon`)
}))

Promise.all(icons.map(icon => {
  const svg = simple[icon.key].svg
  const component = componentTemplate(icon.pascalCasedComponentName, svg)
  const filepath = `./src/components/${icon.pascalCasedComponentName}.vue`
  return fs.ensureDir(path.dirname(filepath))
    .then(() => fs.writeFile(filepath, component, 'utf8'))
})).then(() => {
  const main = icons
    .map(icon => `export { default as ${icon.pascalCasedComponentName} } from './components/${icon.pascalCasedComponentName}.vue'`)
    .join('\n\n')
  return fs.outputFile('./src/index.js', main, 'utf8')
});
