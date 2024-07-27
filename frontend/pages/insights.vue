<template>

    <Header />
    <div class="container mx-auto p-4">
        <LoadingScreen v-if="isLoading" />
        <div v-else-if="scanResult">
            <h1 class="text-2xl font-bold mb-4">Scan Results</h1>
            <pre>{{ JSON.stringify(scanResult, null, 2) }}</pre>
        </div>
    </div>
</template>

<script setup>
const route = useRoute()
const isLoading = ref(true)
const scanResult = ref(null)

async function pollForResult(trackingId, maxAttempts = 20, interval = 2000) {
  for (let attempt = 0; attempt < maxAttempts; attempt++) {
    try {
      const statusResult = await $fetch(`/backend/status?id=${trackingId}`, {
        method: 'GET'
      })
      
      console.log('Status result:', statusResult); // For debugging

      if (!statusResult.loading) {
        const result = await $fetch(`/backend/result?id=${trackingId}`, {
          method: 'GET'
        })
        scanResult.value = result
        isLoading.value = false
        return
      }
    } catch (error) {
      console.error('Error in polling:', error);
      if (error.response?.status === 404) {
        throw new Error('Tracking ID not found');
      }
    }

    // Wait before next check
    await new Promise(resolve => setTimeout(resolve, interval))
  }

  throw new Error('Processing Timed Out!')
}

onMounted(async () => {
    const trackingId = route.query.trackingId
    if (trackingId) {
        try {
            await pollForResult(trackingId)
        } catch (error) {
            console.error('Error polling for result:', error)
        }
    } else {
        isLoading.value = false
    }
})
</script>