<template>
  <div class="player">
    <div>
      <img src="/cover.png" alt="cover" class="cover">
      <div class="title">Title</div>
    </div>
    <div class="controls">
      <IconsPlay v-if="paused" @click="play" />
      <IconsPause v-if="!paused" @click="pause" />
      <IconsSkip @click="skip" />
    </div>
    <div class="progress"></div>
  </div>
</template>

<script lang="ts" setup>
const paused = ref(true)

function play(){
  paused.value = false;
}

function pause() {
  paused.value = true;
}
</script>

<style scoped lang="scss">
@use '~/assets/styles/components/player';
</style>