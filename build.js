const path = require('path')
const writtenNumber = require('written-number');
const fs = require('fs-extra')
const simple = require('simple-icons/icons')
const {pascalCase} = require("pascal-case");
const componentTemplate = (name, svg) => `
<template>
    ${svg.replace(/<svg([^>]+)>/, '<svg :width="size" :height="size" role="img" viewBox="0 0 24 24" v-bind="$attrs" >')}
</template>
<script lang="ts">
import {  defineComponent } from "vue";
export default defineComponent({
  name: '${name}',
  props: {
    size: {
      type: String,
      default: '24'
    }
  },
  setup(props) {
    if (props.size === undefined){
      return { size: '24' }
    }
    const size = props.size.slice(-1) === 'x'
      ? props.size.slice(0, props.size.length -1) + 'em'
      : parseInt(props.size) + 'px';
    return {size}
  }
});
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
    .map(icon => `import ${icon.pascalCasedComponentName} from '@/components/${icon.pascalCasedComponentName}.vue'`)
    .join('\n\n');
  const ex = "export {" + icons.map(icon => icon.pascalCasedComponentName).join(', ') + "}"
  const final = main + '\n\n' + ex;
  return fs.outputFile('./src/main.ts', final, 'utf8')
});
