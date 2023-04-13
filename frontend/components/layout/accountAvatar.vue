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
  <div id="layout-account-avatar">
    <v-hover v-slot="{ isHovering, props }">
      <div v-bind="props">
        <v-avatar class="pointer" @click="$router.push(localePath('/account'))">
          <v-img alt="account" :src="avatar" v-if="loggedIn"/>
          <v-img alt="account" src="https://minotar.net/avatar/MHF_Steve" v-else/>
        </v-avatar>

        <v-fade-transition>
          <v-avatar class="pointer" style="background: rgba(0, 0, 0, 0.3); z-index: 100;"
                    v-if="!loggedIn" @click="openLoginDialog" v-show="isHovering">
            <v-icon icon="mdi-login"/>
          </v-avatar>
        </v-fade-transition>
      </div>
    </v-hover>
  </div>
</template>

<script lang="ts" setup>
import {computed, openLoginDialog} from "#imports";
import {useAuthStore} from "~/stores/auth";

const auth = useAuthStore();

const loggedIn = computed(() => auth.isLoggedIn());
const avatar = computed(() => {
  if (auth.isLoggedIn() && auth.account?.uuid)
    return `https://minotar.net/avatar/${auth.account.uuid}`
  else
    return "https://minotar.net/avatar/MHF_Steve"
})
</script>

<style lang="sass">
#layout-account-avatar
  position: relative

  div
    .v-avatar
      position: absolute
      top: -20px
      left: 0
</style>
