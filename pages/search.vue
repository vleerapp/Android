<template>
  <h1 :class="{ 'title': true, 'searching': searching }">Search</h1>
  <div class="search-cover"></div>
  <div :class="{ 'search-container': true, 'searching': searching }">
    <IconsSearch />
    <input ref="searchInput" class="input" spellcheck="false" type="text" v-model="searchTerm" @input="handleInput()"
      @focus="searching = true" @focusout="handleFocusOut()" @blur="searching = searchTerm.length != 0"
      placeholder="Search" />
  </div>
  <div :class="{ 'results': true, 'searching': searching }">
    <div class="song" v-for="(song, index) in searchResults" :class="{ 'first-result': index === 0 }"
      @click="handleSongClick(song)">
      <img :src="song.thumbnail" :alt="song.title" class="cover" loading="lazy" />
      <div class="info">
        <div>
          <div class="title" ref="titleRefs">{{ trunance(song.title) }}</div>
          <div class="artist" ref="artistRefs">{{ trunance(song.uploaderName) }}</div>
        </div>
        <IconsDots class="dots" />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { MusicSearchResponseItem, Song } from '~/types/types';

const searching = ref(false);
const searchTerm = ref("");
const searchResults = ref<MusicSearchResponseItem[]>([]);
const searchInput = ref<HTMLInputElement | null>(null);
const titleRefs = ref<HTMLElement[]>([]);
const artistRefs = ref<HTMLElement[]>([]);
let searchTimeout: ReturnType<typeof setTimeout>;

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
      .filter((item: MusicSearchResponseItem) => item.type !== 'channel')
      .map((item: MusicSearchResponseItem) => ({
        url: item.url,
        title: item.title,
        thumbnail: item.thumbnail,
        uploaderName: item.uploaderName,
        uploaderAvatar: item.uploaderAvatar,
        duration: item.duration,
        durationFormatted: `${Math.floor(item.duration / 60)}:${item.duration % 60 < 10 ? '0' : ''}${item.duration % 60}`
      }));
  } catch (error) {
    console.error("Failed to fetch songs:", error, searchTerm.value);
    searchResults.value = [];
  }
}

async function handleSongClick(song: MusicSearchResponseItem) {
  const match = song.url.match(/(?:\/watch\?v=)([^&]+)/)! as RegExpMatchArray;

  if (!match || !match[1]) {
    console.error("No valid ID found in the URL.");
    return;
  }

  const videoId = match[1];

  var songData: Song = {
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

function trunance(text: string) {
  const width = (window.innerWidth - 140) / 9;
  return text.length > width ? text.substring(0, width - 3).trim() + "..." : text;
}
</script>

<style scoped lang="scss">
@use '~/assets/styles/pages/search';
</style>
