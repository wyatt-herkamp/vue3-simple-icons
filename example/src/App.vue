<template>
  <div id="app">
    <header class="header">
      <div class="container">
        <h1 class="hero-heading">vue3-simple-icons</h1>
        <h2 class="desc">
          <a href="https://simpleicons.org/" target="_blank">Simple Icons</a> as Vue components.
        </h2>
        <h3 class="desc">Github <a href="https://github.com/wyatt-herkamp/vue3-simple-icons">vue3-simple-icons</a></h3>
      </div>
    </header>
    <div class="container">
      <div class="search-bar">
        <input
          type="text"
          class="search-input"
          v-model="keyword"
          :placeholder="`Search ${icons.length} icons...`"
        />
      </div>
      <div class="icons">
        <div
          class="icon"
          v-for="icon in filteredIcons"
          :key="icon"
        >
          <component :is="icon" class="icon-svg"></component>
          <span>{{ icon }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script>

import * as icons from "vue3-simple-icons";

export default {
  data() {
    return {
      icons: Object.keys(icons),
      keyword: "",
      hoverIcon: "",
      hoverSize: "",
      year: new Date().getFullYear(),
      exampleSizes: ["24", "1x", "1.5x", "2x", "3x", "4x"],
    };
  },
  computed: {
    filteredIcons() {
      const keyword = this.keyword.trim().toLowerCase();
      if (!keyword) return this.icons;

      return this.icons.filter((name) => {
        return name.toLowerCase().indexOf(keyword) > -1;
      });
    },

  },
  methods: {

  },
  components: {
    ...icons,
  },
};
</script>


<style>
@import url('https://rsms.me/inter/inter.css');



a.credit {
  padding: 4px 8px;
  background: #004080;
  text-align: center;
  width:100%;
  display:block;
  position:fixed;
  top:0;
  left:0;
  right:0;
  z-index:10000;
}

a.credit:hover {
  text-decoration: none;
}

a.credit span {
  font-size:80%;
  line-height:0.9;
  color: rgba(255,255,255,0.9);
  display:inline-block;
  transform: translateY(-1px);
  border-bottom: 1px solid rgba(255,255,255,0.75);
}

body {
  margin: 0;
  font: 14px/1.4 "Inter", "Helvetica Neue", Helvetica, Arial, sans-serif;
}

* {
  box-sizing: border-box;
}

a {
  color: #333;
  text-decoration: none;
}

a:hover {
  text-decoration: underline;
}

.container {
  max-width: 1080px;
  margin: 0 auto;
  padding: 0 10px;
}


</style>

<style scoped>
.header {
  background: #263238;
  padding: 70px 0 40px;
}

.desc {
  color: white;
  font-weight: 400;
}

.desc a {
  color:#ffffff !important;
  text-decoration: underline;
}

.hero-heading {
  color: #ffffff;
  margin: 0;
  font-size: 2rem;
  font-weight: 500;
}

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
</style>
