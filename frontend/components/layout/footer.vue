<!--
  -
  - The MIT License (MIT)
  -
  - Copyright (c) 2023 Fritz Ochsmann
  -
  - Permission is hereby granted, free of charge, to any person obtaining a copy
  - of this software and associated documentation files (the "Software"), to deal
  - in the Software without restriction, including without limitation the rights
  - to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
  - copies of the Software, and to permit persons to whom the Software is
  - furnished to do so, subject to the following conditions:
  -
  - The above copyright notice and this permission notice shall be included in all
  - copies or substantial portions of the Software.
  -
  - THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
  - IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
  - FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
  - AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
  - LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
  - OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
  - SOFTWARE.
  -
  -->

<template>
  <v-footer class="d-flex text-center flex-column pt-8">
    <div>
      <v-btn v-for="social in socials" :key="social.icon" variant="text" rounded>
        <NuxtLink :href="social.link" target="_blank">
          <v-icon :icon="social.icon" size="x-large" />
        </NuxtLink>
      </v-btn>
    </div>

    <v-row class="pa-5">
      <v-col cols="12">
        &copy; MyPlayPlanet.net - {{ new Date().getFullYear() }}
      </v-col>
    </v-row>
  </v-footer>
</template>

<script lang="ts" setup>
const socials = [
  { link: "", icon: "mdi-youtube" },
  { link: "", icon: "mdi-twitter" },
  { link: "", icon: "mdi-steam" },
]
</script>

<style lang="sass" scoped>
.v-footer
  width: 100%
  background: linear-gradient(180deg, rgba(73, 0, 30, 1) 0%, rgba(140, 0, 0, 1) 100%)
</style>