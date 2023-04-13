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
  <v-app>
    <LayoutAppBar/>

    <div id="content">
      <NuxtLayout>
        <v-card variant="tonal">
          <v-card-title id="error-title-bg">
            <span class="text-h2 font-intro-inline">
              Error {{ error.statusCode }}
            </span>
          </v-card-title>

          <v-card-subtitle>
            {{ error.statusMessage }}
          </v-card-subtitle>

          <v-card-text>
              <span class="text-h5 font-weight-thin">
                {{ error.message }}
              </span>

            <div id="error-stack" class="pa-5 mt-8" v-html="error.stack" />
          </v-card-text>
        </v-card>
      </NuxtLayout>
    </div>
  </v-app>
</template>

<script lang="ts" setup>
const props = defineProps({error: {type: Object, required: true}})
</script>

<style lang="sass" scoped>
#error-title-bg
  background: linear-gradient(180deg, rgba(73, 0, 30, 1) 0%, rgba(140, 0, 0, 1) 100%)

#error-stack
  border: 1px solid grey
  border-radius: 5px
</style>