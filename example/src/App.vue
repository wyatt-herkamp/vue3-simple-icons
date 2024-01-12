<template>
  <div id="app">
    <SiteHeader />
    <div class="container">
      <div class="search-bar">
        <input
          type="text"
          class="search-input"
          v-model="keyword"
          :placeholder="`Search ${numberOfIcons} icons...`"
        />
      </div>
      <div class="icons">
        <IconElement v-for="icon in filteredIcons" :key="icon.componentName" :icon="icon" />
      </div>
    </div>
  </div>
  <notifications />

</template>

<script lang="ts" setup>
import { ref, computed } from 'vue'
import SiteHeader from '@/components/SiteHeader.vue'
import IconElement from '@/components/IconElement.vue'
import info from "@/../../simple-icons.json"
interface ComponentInfo{
  originalTitle: string;
  componentName: string;
  slug: string;
}
const components: Array<ComponentInfo> = info.components;
const keyword = ref('')
const filteredIcons = computed(() => {
  const keywordValue = keyword.value.trim().toLowerCase()
  if (!keywordValue) return components

  return components.filter((component) => {
    return component.componentName.toLowerCase().indexOf(keywordValue) > -1
  })
})
const numberOfIcons = computed(() => components.length)
</script>

<style scoped lang="scss">
.search-bar {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100px;
}

.search-input {
  padding: 10px;
  outline: none;
  width: 100%;
  font-size: 1rem;
  border: 1px solid #e2e2e2;
  border-radius: 3px;
}

.search-input:focus {
  border-color: #ccc;
}

@media screen and (max-width: 768px) {
  .icon {
    width: 100%;
  }
}
@media screen and (min-width: 768px) {
  .container {
    max-width: 1080px;
    margin: 0 auto;
    padding: 0 10px;
  }

.icons{
    display: flex;
    flex-wrap: wrap;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    align-content: center;
    gap: 10px;
}
}
</style>
