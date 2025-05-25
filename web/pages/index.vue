<template>
  <div class="w-dvw h-dvh flex items-center flex-col">
    <hgroup
      class="flex items-center justify-center flex-col mb-2 bg-black text-white w-full pb-8"
    >
      <h1>
        <img
          class="max-h-48 max-w-screen"
          src="~/assets/img/banner.svg"
          alt="Nintendo Yesterday"
        />
      </h1>
      <h2 class="text-2xl">An archive of <i>Nintendo Today</i></h2>
    </hgroup>

    <div
      class="flex items-center justify-center gap-4 m-4 self-center justify-self-center"
    >
      <NuxtLink to="/events/"><h3 class="text-2xl">Events</h3></NuxtLink>-
      <NuxtLink to="/tags/"><h3 class="text-2xl">Tags</h3></NuxtLink>-
      <NuxtLink to="/calendars/"><h3 class="text-2xl">Calendar animations</h3></NuxtLink>-
      <NuxtLink to="/content/"><h3 class="text-2xl">All content</h3></NuxtLink>
    </div>
    <div
      class="mt-auto flex items-center justify-center flex-col bg-black text-white w-full p-2"
    >
      This site is a work in progress. It is not affiliated with Nintendo in any way.
    </div>
  </div>
</template>
