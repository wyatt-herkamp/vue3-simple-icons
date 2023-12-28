<template>
  <header class="header">
    <div class="container">
      <h1 class="hero-heading">vue3-simple-icons</h1>
      <h2 class="desc">
        <a href="https://simpleicons.org/" target="_blank">Simple Icons</a> as Vue components.
      </h2>
      <h3 class="desc">
        Github <a href="https://github.com/wyatt-herkamp/vue3-simple-icons">vue3-simple-icons</a>
      </h3>
      <a href="https://www.npmjs.com/package/vue3-simple-icons">
        <img alt="Current Node Version" src="https://badge.fury.io/js/vue3-simple-icons.svg"></a>
    </div>
  </header>
</template>
<script></script>

<style scoped lang="scss">
.header {
  background: #263238;
  padding: 70px 0 40px;
}

.desc {
  color: white;
  font-weight: 400;
}

.desc a {
  color: #ffffff !important;
  text-decoration: underline;
}

.hero-heading {
  color: #ffffff;
  margin: 0;
  font-size: 2rem;
  font-weight: 500;
}
.container {
  max-width: 1080px;
  margin: 0 auto;
  padding: 0 10px;
}
</style>
