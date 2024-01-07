import { getIconSlug } from "simple-icons/sdk";
import { Brand } from "./types.ts";
const componentScript = await Deno.readTextFile("template.vue");
import * as changeCase from "change-case";
import n2words from "n2words";

export function buildComponent(iconSVG: string): string {
  const result = `
<template>
    ${iconSVG.replace(
      /<svg([^>]+)>/,
      '<svg :width="finalSize" :height="finalSize" role="img" viewBox="0 0 24 24" v-bind="$attrs" >'
    )}
</template>
${componentScript}`;

  return result;
}

/**
 * If Brand.title is all AlphaNumeric characters and spaces convert to PascalCase
 * If Brand.Title is not use Brand slug
 * @param icon Brand
 * @returns string
 *
 */
export function getComponentName(icon: Brand): string {
  if (icon.title.match(/^[a-zA-Z ]+$/)) {
    if (icon.slug) {
      const slug = changeCase.pascalCase(icon.slug);
      console.info("Using slug for component name", slug);
      return `${slug}Icon`;
    }
    const name = changeCase.pascalCase(icon.title);
    return `${name}Icon`;
  }
  let title = getIconSlug(icon);
  const beginningNumbers = icon.title.replace(/[^\d].*/, "");
  if (beginningNumbers.length) {
    const numberInEnglish = n2words(beginningNumbers, {
      lang: "en",
    })
      .split(" ")
      .filter((word) => word !== "and")
      .join("-");

    const numberSlug = `${numberInEnglish}-`;
    title = title.replace(beginningNumbers, numberSlug);
  }
  const finalTile = changeCase.pascalCase(title);
  return `${finalTile}Icon`;
}
