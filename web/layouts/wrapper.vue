<template>
  <div class="w-dvw h-dvh flex items-center flex-col">
    <hgroup
      class="flex items-center justify-center mb-2 bg-black text-white w-full gap-8 h-26"
    >
      <NuxtLink to="/">
        <h1>
          <img
            class="max-h-26 max-w-screen"
            src="~/assets/img/banner.svg"
            alt="Nintendo Yesterday"
          />
        </h1>
      </NuxtLink>
      <h2 class="text-xl">An archive of <i>Nintendo Today</i></h2>
    </hgroup>

    <slot></slot>
    <div
      class="mt-auto flex items-center justify-center flex-col bg-black text-white w-full p-2"
    >
      This site is a work in progress. It is not affiliated with Nintendo in any
      way.
    </div>
  </div>
</template>
