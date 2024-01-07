<script setup lang="ts">
import { computed,  unref } from 'vue'
//${svg.replace(/<svg([^>]+)>/, '<svg :width="size" :height="size" role="img" viewBox="0 0 24 24" v-bind="$attrs">')}
export interface Props {
  size?: number | string
}
const props = withDefaults(defineProps<Props>(), {
  size: 24
})
const finalSize = computed(() => {
  const size = unref(props.size)
  return typeof size === 'number' ? `${size}px` : size
})
</script>
