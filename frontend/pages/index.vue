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
      <ButtonPrimary @click="openFileDialog">
        Upload File
      </ButtonPrimary>
    </div>
  </div>
  <FileUploadDialog ref="fileDialog" @file-scanned="handleScannedFile" />
</template>

<script setup>

const fileDialog = ref(null)
const scanResult = ref(null)

function openFileDialog() {
  fileDialog.value.openDialog()
}

async function handleScannedFile(formData) {
  try {
    scanResult.value = await $fetch('/backend/process_file', {
      method: 'POST',
      body: formData
    })

    const tracking_id = scanResult.value.tracking_id
    
    // Process the scan result (e.g., update UI, etc.)
  } catch (error) {
    console.error('Error scanning file:', error)
    // Handle error (e.g., show error message to user)
  } 
}

</script>
