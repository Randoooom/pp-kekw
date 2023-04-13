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
  <v-row>
    <v-col cols="12">
      <v-toolbar class="bg-accent rounded-sm">
        <v-toolbar-title>
          {{ $t("nav.news") }}

          <v-card-subtitle class="pa-0">
            {{ $t("paging.result", {count: 0}) }}
          </v-card-subtitle>
        </v-toolbar-title>
      </v-toolbar>
    </v-col>

    <v-col cols="12" sm="6" md="6" v-for="entry in news" :key="entry.id">
      <v-card variant="tonal">
        <v-card-title class="bg-gradient">
          {{ entry.title }}
        </v-card-title>

        <v-card-subtitle>
          {{ moment.utc(entry.createdAt).toDate().toLocaleString() }}
        </v-card-subtitle>

        <v-card-actions>
          <v-switch v-model="entry.shown" color="green-darken-3" :label="$t('news.visible')"/>

          <v-spacer/>

          <v-btn variant="text" color="accent">
            {{ $t("form.edit") }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-col>

    <v-col cols="12">
      <v-btn variant="tonal" color="green" block :disabled="!auth.hasPermission('news.create')">
        {{ $t("news.create") }}
      </v-btn>
    </v-col>

    <v-col cols="12" v-if="pages > 1">
      <v-pagination v-model="page" :length="pages" class="mt-5"/>
    </v-col>
  </v-row>
</template>

<script lang="ts" setup>
import {definePageMeta, ref, useRoute, useRouter, watch} from "#imports";
import {News} from "~/composables/types";
import Fetch from "~/composables/fetch";
import moment from "moment";
import {useAuthStore} from "~/stores/auth";

const route = useRoute();
const router = useRouter();
const auth = useAuthStore();

const page = ref((+route.query.page || 1) as number);
const pages = ref(0);
const total = ref(0)
const news = ref([] as News[])

watch(page, async (page: number) => {
  await fetchNews();
  await router.push({query: {page}})
}, {immediate: true})

/**
 * fetch a page of news
 */
async function fetchNews() {
  await Fetch.getPage<News>("/news", page.value)
      .then((response) => {
        pages.value = response._data!.pages;
        total.value = response._data!.total;
        news.value = response._data!.data;
      })
}

definePageMeta({
  auth: true,
  permission: "news.get.all"
})
</script>
