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
  <FormDialog :title="$t('auth.login.title')">
    <template v-if="!totpNeeded">
      <v-col cols="12">
        <v-text-field v-model="username" :rules="[required()]" color="white" :label="$t('auth.username')"
                      type="text"/>
      </v-col>

      <v-col cols="12">
        <FormPasswordInput v-model="password" color="white" :label="$t('auth.password')" @keyup.enter="login"/>
      </v-col>
    </template>

    <v-col v-else cols="12">
      <!-- `v-otp-input` does not exist currently -->
      <v-text-field v-model="token" :rules="[required()]" color="white" :label="$t('auth.2fa')" type="text"
                    @keyup.enter="login"/>
    </v-col>

    <template #actions>
      <v-btn color="error" variant="text" @click="close">
        {{ $t("cancel") }}
      </v-btn>

      <v-spacer/>

      <v-btn :disabled="disabled(username, password)" :loading="loading" color="green-darken-3" variant="text" @click="login">
        {{ $t("auth.login.execute") }}
      </v-btn>
    </template>
  </FormDialog>
</template>

<script lang="ts" setup>
import {useDialogStore} from "~/stores/dialog";
import {ref, required, disabled, useI18n, useRouter} from "#imports";
import {AuthenticationError, AuthenticationErrorType, useAuthStore} from "~/stores/auth";
import {useEmitter} from "~/stores/emitter";
import {localePath} from "vue-i18n-routing";

const username = ref("");
const password = ref("");
const token = ref("");
const loading = ref(false);
const totpNeeded = ref(false);

const props = defineProps({
  id: {type: String, required: true}
})
const {t} = useI18n();
const emitter = useEmitter();

/**
 * perform the login request
 */
async function login() {
  loading.value = true;
  // try to log in
  await useAuthStore().login({
    username: username.value,
    password: password.value,
    token: token.value,
    captcha: ""
  }).then(() => {
    loading.value = false;

    emitter.emit({
      color: "success",
      content: t("auth.login.success"),
      buttonText: t("emit.button"),
      icon: "mdi-check",
      callback: async () => emitter.clear()
    })

    // login was successful, so we just close the dialog and redirect to the dashboard
    useDialogStore().closeDialog(props.id!)
    useRouter().push(localePath("/account"))
  })
      .catch((error: AuthenticationError) => {
        loading.value = false;
        // login failed
        if (error.type === AuthenticationErrorType.TotpNeeded) {
          totpNeeded.value = true

          emitter.emit({
            color: "warning",
            content: t("auth.login.totp"),
            buttonText: t("emit.button"),
            icon: "mdi-alert",
            callback: async () => emitter.clear()
          })
        } else
          emitter.emit({
            color: "error",
            content: t("auth.login.failed"),
            buttonText: t("emit.button"),
            icon: "mdi-alert-circle-outline",
            callback: async () => emitter.clear()
          })
      })
}

/**
 * close the dialog
 */
function close() {
  useDialogStore().closeDialog(props.id!)
}
</script>
