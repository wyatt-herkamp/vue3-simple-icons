<template>

  <div class="icon" @click="copyToClipboard">
    <component :is="iconComponent" class="icon-svg"></component>
    <div class="iconInfo">
      <span>Name: {{ icon.originalTitle }}</span>
      <span>Component: {{ icon.componentName }}</span>
      <span>Original Slug: {{ icon.slug }}</span>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { computed, defineProps } from 'vue'
import * as icons from 'vue3-simple-icons'
import { useNotification } from '@kyvg/vue3-notification'
import type { PropType } from 'vue';

const { notify } = useNotification()

const props = defineProps({
  icon: {
    type: Object as PropType<{
        originalTitle: string;
      componentName: string;
      slug: string;
    }>,
    required: true
  }
})
const iconComponent = computed(() => icons[props.icon.componentName])
function copyToClipboard() {
    navigator.clipboard.writeText(props.icon.componentName);
    console.log(props.icon)
    notify({
    title: "Copied to clipboard!",
    text: `${props.icon.componentName} copied to clipboard!`,
  });
}
</script>
<style scoped lang="scss">
.icons {
  display: flex;
  flex-wrap: wrap;
}

.icon {
  display: flex;
  align-items: center;
  padding: 10px;
  width: 25%;
  border-radius: 3px;
  cursor: pointer;
}

.icon:hover {
  background: #f1f5ff;
}
.iconInfo {
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.icon-svg {
  margin-right: 10px;
}
</style>

