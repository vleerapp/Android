<template>
  <div class="cover-plate"></div>
  <Navbar />
  <Player />
  <div class="container">
    <slot />
  </div>
</template>

<style lang="scss">
@use "~/assets/styles/variables" as v;

.container {
  padding-inline: 20px;
  margin-top: 8px;
  padding-bottom: 134px;

  .title {
    font-size: 36px;
  }
}

.cover-plate {
  width: 100%;
  position: fixed;
  height: 114px;
  bottom: 0;
  background-color: v.$background;
  z-index: 2;
  overflow: hidden;
}
</style>
