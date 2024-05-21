<template>
  <div class="navbar">
    <NuxtLink to="/" class="link home">
      <div class="svg-default">
        <IconsNavbarHomeOutline />
      </div>
      <div class="svg-active">
        <IconsNavbarHomeFill />
      </div>
      Home
    </NuxtLink>
    <NuxtLink to="/songs" class="link songs">
      <div class="svg-default">
        <IconsNavbarSongsOutline />
      </div>
      <div class="svg-active">
        <IconsNavbarSongsFill />
      </div>
      Songs
    </NuxtLink>
    <NuxtLink to="/library" class="link library">
      <div class="svg-default">
        <IconsNavbarLibrary />
      </div>
      <div class="svg-active">
        <IconsNavbarLibrary />
      </div>
      Library
    </NuxtLink>
    <NuxtLink to="/settings" class="link settings">
      <div class="svg-default">
        <IconsNavbarSettingsOutline />
      </div>
      <div class="svg-active">
        <IconsNavbarSettingsFill />
      </div>
      Settings
    </NuxtLink>
    <NuxtLink to="/search" class="link search">
      <div class="svg-default">
        <IconsNavbarSearchOutline />
      </div>
      <div class="svg-active">
        <IconsNavbarSearchFill />
      </div>
      Search
    </NuxtLink>
  </div>
</template>

<style scoped lang="scss">
@use '~/assets/styles/components/player';
</style>