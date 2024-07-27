<template>
  <Header />

  <div class="m-16 bg-blue-200; lg:mt-32">
    <div class="flex flex-col gap-4 items-center lg:items-start">
      <div class="text-2xl text-center font-bold md:text-4xl lg:text-6xl lg:w-1/2 lg:text-left lg:leading-12">
        <h1>Gain insights of your project
          <span class="text-[#8900CA]">dependecies.</span>
        </h1>
      </div>
      <p class="text-center md:text-xl lg:text-2xl lg:text-left">
        For enthusiasts to contribute <br />
        effectively to the open-source ecosystem.</p>
      <p class="text-lg"></p>
      <ButtonPrimary @click="$router.push('/')">
        Upload File
      </ButtonPrimary>
    </div>


  </div>
</template>

<script setup lang="ts">
</script>
