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
  <v-card>
    <v-toolbar color="accent">
      <v-toolbar-title>
        {{ $t("dashboard.title") }}

        <v-card-subtitle class="pa-0">
          {{ $t("dashboard.subtitle", {username: account.username}) }}
        </v-card-subtitle>
      </v-toolbar-title>
    </v-toolbar>

    <div :class="isXs ? '' : 'd-flex flex-row'">
      <v-tabs direction="vertical" v-model="tab" color="accent">
        <v-tab value="general">
          <v-icon start icon="mdi-cog"/>
          {{ $t("dashboard.general") }}
        </v-tab>

        <v-tab value="security">
          <v-icon start icon="mdi-shield-lock-outline" />
          {{ $t("dashboard.security") }}
        </v-tab>

        <v-tab value="schematics">
          <v-icon start icon="mdi-cube-outline" />
          {{ $t("dashboard.schematics") }}
        </v-tab>
      </v-tabs>

      <v-window v-model="tab" class="mt-5" style="width: 100%">
        <v-window-item value="general">
          <DashboardAccountGeneral />
        </v-window-item>

        <v-window-item value="security">
          <DashboardAccountSecurity />
        </v-window-item>
      </v-window>
    </div>
  </v-card>
</template>

<script lang="ts" setup>
import {definePageMeta, ref} from "#imports";
import {useAuthStore} from "~/stores/auth";

const account = useAuthStore().account!;
const tab = ref("general");

definePageMeta({
  auth: true
})
</script>
