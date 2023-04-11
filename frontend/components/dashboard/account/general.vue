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
  <v-card flat>
    <v-card-text>
      <v-row>
        <v-col cols="12">
          <v-text-field :label="$t('auth.username')" color="white" :rules="[required()]" v-model="username"/>
        </v-col>
      </v-row>
    </v-card-text>

    <v-card-actions>
      <v-spacer/>
      <v-btn variant="text" color="accent" @click="logout">
        Ausloggen
      </v-btn>
    </v-card-actions>
  </v-card>
</template>

<script lang="ts" setup>
import {useAuthStore} from "~/stores/auth";
import {required} from "~/composables/form";
import {computed, ref, useI18n, useRouter, watch} from "#imports";
import _ from "lodash"
import {useEmitter} from "~/stores/emitter";
import Fetch from "~/composables/fetch"
import {localePath} from "vue-i18n-routing";

const emitter = useEmitter();
const {t} = useI18n();

const account = computed(() => useAuthStore().account)
const username = ref(_.clone(account.value!.username))

watch(() => account.value!.username !== username.value, (newValue: boolean, oldValue: boolean) => {
  if (newValue && !oldValue) {
    emitter.emit({
      color: "warning",
      icon: "mdi-alert",
      content: t("form.changed"),
      buttonText: t("form.save"),
      callback: updateUsername
    })
  } else
    emitter.clear()
})

/**
 * perform the put request
 */
async function updateUsername() {
  await Fetch.put(`/account/${account.value!.id}`, {
    body: {
      username: username.value
    }
  }).then(() => useAuthStore().fetchAccount())
}

/**
 * logout
 */
async function logout() {
  await useAuthStore().logout()
      .then(async () => {
        useEmitter().emit({
          icon: "mdi-check",
          color: "success",
          content: t("auth.logout.success"),
          callback: async () => useEmitter().clear(),
          buttonText: t("emit.button")
        })

        await useRouter().push("/")
      })
}
</script>
