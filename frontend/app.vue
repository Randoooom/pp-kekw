<template>
  <v-app>
    <LayoutAppBar/>

    <div id="content">
      <NuxtLayout>
        <NuxtPage/>
      </NuxtLayout>
    </div>
  </v-app>
</template>

<style lang="sass" scoped>
#content
  margin-top: 64px
  min-height: calc(100vh - 64px)
</style>
