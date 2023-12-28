<template>

  <div class="icon" @click="copyToClipboard">
    <component :is="iconComponent" class="icon-svg"></component>
    <span>{{ icon }}</span>
  </div>
</template>

<script lang="ts" setup>
import { computed, defineProps } from 'vue'
import * as icons from 'vue3-simple-icons'
import { useNotification } from '@kyvg/vue3-notification'

const { notify } = useNotification()

const props = defineProps({
  icon: {
    type: String,
    required: true
  }
})
const iconComponent = computed(() => icons[props.icon])
function copyToClipboard() {
    navigator.clipboard.writeText(props.icon);
    console.log(props.icon)
    notify({
    title: "Copied to clipboard!",
    text: `${props.icon} copied to clipboard!`,
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

.icon-svg {
  margin-right: 10px;
}
</style>

