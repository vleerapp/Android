<template>
  <h1 :class="{ 'title': true, 'searching': searching }">Search</h1>
  <div class="search-cover"></div>
  <div :class="{ 'search-container': true, 'searching': searching }">
    <IconsSearch />
    <input ref="searchInput" class="input" spellcheck="false" type="text" v-model="searchTerm" @input="handleInput()"
      @focus="searching = true" @focusout="handleFocusOut()" @blur="searching = searchTerm.value.length != 0"
      placeholder="Search" />
  </div>
  <div :class="{ 'results': true, 'searching': searching }">
    <div class="song" v-for="(song, index) in searchResults" :class="{ 'first-result': index === 0 }"
      @click="handleSongClick(song)">
      <img :src="song.thumbnail" :alt="song.title" class="cover" loading="lazy" />
      <div class="info">
        <div>
          <div class="title" ref="titleRefs">{{ truncateText(song.title, 'title') }}</div>
          <div class="artist" ref="artistRefs">{{ truncateText(song.uploaderName, 'artist') }}</div>
        </div>
        <IconsDots class="dots" />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted, nextTick } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const searching = ref(false);
const searchTerm = ref("");
const searchResults = ref<[]>([]);
let searchTimeout: ReturnType<typeof setTimeout>;
const searchInput = ref<HTMLInputElement | null>(null);
const titleRefs = ref<HTMLElement[]>([]);
const artistRefs = ref<HTMLElement[]>([]);

function handleInput() {
  clearTimeout(searchTimeout);
  searchTimeout = setTimeout(() => {
    if (searchTerm.value.trim()) {
      searchSongs();
    } else {
      searchResults.value = [];
    }
  }, 500);
}

async function searchSongs() {
  if (searchTerm.value === "") {
    searchResults.value = [];
    return;
  }

  try {
    const response = await fetch(`https://pipedapi.r4fo.com/search?q=${encodeURIComponent(searchTerm.value)}&filter=music_songs`);
    const data = await response.json();
    searchResults.value = data.items
      .filter(item => item.type !== 'channel')
      .map(item => ({
        url: item.url,
        title: item.title,
        thumbnail: item.thumbnail,
        uploaderName: item.uploaderName,
        uploaderAvatar: item.uploaderAvatar,
        duration: item.duration,
        durationFormatted: `${Math.floor(item.duration / 60)}:${item.duration % 60 < 10 ? '0' : ''}${item.duration % 60}`
      }));
    truncateAllTexts();
  } catch (error) {
    console.error("Failed to fetch songs:", error, searchTerm.value);
    searchResults.value = [];
  }
}

async function handleSongClick(song) {
  const match = song.url.match(/(?:\/watch\?v=)([^&]+)/)! as RegExpMatchArray;

  if (!match || !match[1]) {
    console.error("No valid ID found in the URL.");
    return;
  }

  const videoId = match[1];

  var songData = {
    id: videoId,
    title: song.title,
    artist: song.uploaderName,
    length: song.duration,
    cover: song.thumbnail.replace(/^https?:\/\/[^\/]+/, ''),
    date_added: formatDate(new Date())
  }

  await invoke("log", { log: "click: " + JSON.stringify(songData) })
  await invoke('download', { url: "https://youtube.com" + song.url, name: videoId + ".webm" });
}

function handleFocusOut() {
  if (searchTerm.value.length === 0) {
    searching.value = false;
  }
}

onMounted(() => {
  if (searchInput.value) {
    searchInput.value.focus();
  }
});

const formatDate = (date: Date) => {
  let year = date.getFullYear();
  let month = (date.getMonth() + 1).toString().padStart(2, '0');
  let day = date.getDate().toString().padStart(2, '0');
  let hours = date.getHours().toString().padStart(2, '0');
  let minutes = date.getMinutes().toString().padStart(2, '0');
  let seconds = date.getSeconds().toString().padStart(2, '0');
  return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
}

const truncateText = (text: string, className: string) => {
  const element = document.createElement('div');
  element.style.position = 'absolute';
  element.style.visibility = 'hidden';
  element.style.whiteSpace = 'nowrap';
  element.className = className;
  element.innerText = text;
  document.body.appendChild(element);

  let truncatedText = text;
  while (element.scrollWidth > element.clientWidth) {
    truncatedText = truncatedText.slice(0, -1);
    element.innerText = truncatedText + '...';
  }

  document.body.removeChild(element);
  return truncatedText;
}

const truncateAllTexts = () => {
  titleRefs.value.forEach((el, index) => {
    el.innerText = truncateText(searchResults.value[index].title, 'title');
  });
  artistRefs.value.forEach((el, index) => {
    el.innerText = truncateText(searchResults.value[index].uploaderName, 'artist');
  });
}
</script>

<style scoped lang="scss">
@use '~/assets/styles/pages/search';
</style>
