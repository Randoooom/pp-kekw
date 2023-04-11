<template>
  <v-app>
    <LayoutAppBar/>

    <div id="content">
      <NuxtLayout>
        <NuxtPage/>
      </NuxtLayout>
    </div>

    <LayoutFooter/>

    <component v-for="dialog in dialogs" :key="dialog.id" :is="dialog.component" :id="dialog.id"
               :options="dialog.options"/>
    <Teleport to="body">
      <v-fade-transition>
        <v-banner v-if="notification" :color="notification.color" :icon="notification.icon"
                  id="banner-emitter">
          <v-banner-text>
            {{ notification.content }}
          </v-banner-text>

          <template #actions>
            <v-btn :loading="notification.loading" @click="notification.callback" :color="notification.color">
              {{ notification.buttonText }}
            </v-btn>
          </template>
        </v-banner>
      </v-fade-transition>
    </Teleport>
  </v-app>
</template>

<script lang="ts" setup>
import {useDialogStore} from "~/stores/dialog";
import {useEmitter} from "~/stores/emitter";
import {computed} from "#imports";

const dialogs = computed(() => useDialogStore().openDialogs);
const notification = computed(() => useEmitter().notification)
</script>

<style lang="sass">
#content
  margin-top: 64px
  min-height: calc(100vh - 64px)

html
  scroll-behavior: smooth

#banner-emitter
  position: fixed
  bottom: 50px
  width: 500px
  left: calc(50vw - 250px)
  z-index: 2500
</style>
