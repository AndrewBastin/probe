
<template>
    <Header />
    <LoadingScreen v-if="isLoading" />
    <div v-else-if="scanResult" class="container mx-auto p-4">
        <h1 class="text-3xl font-bold mb-6 text-violet-300">Scan Results</h1>
        <div class="border border-violet-600 rounded-lg overflow-hidden shadow-lg bg-gray-800 bg-opacity-50">
            <div v-for="(dep, _index) in scanResult.objects" :key="dep.name"
                class="border-b border-violet-600 last:border-b-0">
                <div @click="toggleExpand(dep.name)"
                    class="flex text-violet-200 font-semibold justify-between bg-gray-800 bg-opacity-70 items-center p-5 cursor-pointer hover:bg-gray-700 transition-colors duration-200 ease-in-out shadow-sm">
                    <div class="font-medium text-lg">{{ cleanDependencyName(dep.name) }}</div>
                    <div class="flex justify-between items-baseline gap-6">
                        <div class="flex items-center gap-1">
                            <Icon name="mdi:license" class="text-violet-400" />
                            <span class="text-violet-300">{{ dep.license }}</span>
                        </div>
                        <div v-if="dep.github" class="flex items-center gap-2">
                            <NuxtLink :to="dep.github.repo_url" target="_blank"
                                class="text-violet-400 hover:text-violet-300 underline">
                                <Icon name="mdi:github"/>
                            </NuxtLink>
                        </div>
                    </div>
                </div>
                <div v-if="expandedDeps.includes(dep.name)"
                    class="p-5 bg-gray-800 bg-opacity-30 text-violet-100 shadow-inner"
                    :class="{ 'animate-fadeIn': expandedDeps.includes(dep.name) }">
                    <div v-if="dep.github" class="grid grid-cols-1 gap-4">
                        <div class="bg-gray-700 bg-opacity-50 p-4 rounded-lg shadow-inner">
                            <h3 class="font-semibold text-sm text-violet-300 mb-2">Description</h3>
                            {{ dep.github?.repo_desc || 'No description available' }}
                        </div>

                        <div class="grid grid-cols-1 gap-4 md:grid-cols-3 lg:grid-cols-3">
                            <div class="bg-gray-700 bg-opacity-50 p-4 rounded-lg shadow-inner">
                                <h3 class="font-semibold text-sm text-violet-300 mb-2">Number of contributors</h3>
                                {{ dep.github?.no_of_contributors || 0 }}
                            </div>
                            <div class="bg-gray-700 bg-opacity-50 p-4 rounded-lg shadow-inner">
                                <h3 class="font-semibold text-sm text-violet-300 mb-2">Open Issues</h3>
                                {{ dep.github?.open_issues || 0 }}
                            </div>
                            <div class="bg-gray-700 bg-opacity-50 p-4 rounded-lg shadow-inner">
                                <h3 class="font-semibold text-sm text-violet-300 mb-2">Number of dependencies</h3>
                                {{ getDependencyData(dep.name).length }}
                            </div>
                        </div>
                        <div v-if="getDependencyData(dep.name).length > 0"
                            class="bg-gray-700 bg-opacity-50 p-4 rounded-lg shadow-inner">
                            <h3 class="font-semibold text-sm text-violet-300 mb-2">Dependencies</h3>
                            <ul class="list-square marker:text-violet-300 list-outside ml-4">
                                <li v-for="data in getDependencyData(dep.name).dependencies" :key="data" class="mb-2">
                                    <div class="inline-flex items-center">
                                        <NuxtLink v-if="getGitHubURL(data)" :to="getGitHubURL(data)" target="_blank"
                                            rel="noopener noreferrer"
                                            class="hover:text-violet-300 transition-colors duration-200 flex items-center">
                                            {{ cleanDependencyName(data) }}
                                            <Icon name="mdi:arrow-top-right" class="ml-1 w-4 h-4" />
                                        </NuxtLink>
                                        <span v-else>{{ cleanDependencyName(data) }}</span>
                                    </div>
                                </li>
                            </ul>
                        </div>
                        <div class="bg-gray-700 bg-opacity-50 p-4 rounded-lg shadow-inner">
                            <h3 class="font-semibold text-sm text-violet-300 mb-2">Funding Information</h3>
                            <div v-if="getFundingInfo(dep.name)">
                                <ul class="list-square marker:text-violet-300 list-outside ml-4">
                                    <li v-for="(links, platform) in getFundingInfo(dep.name)" :key="platform" class="mb-2">
                                        <div class="inline-flex items-center">
                                            <NuxtLink
                                                :to="links[0]"
                                                target="_blank" rel="noopener noreferrer"
                                                class="hover:text-violet-300 transition-colors duration-200 flex items-center">
                                                {{ formatPlatformName(platform) }}
                                                <Icon name="mdi:arrow-top-right" class="ml-1 w-4 h-4" />
                                            </NuxtLink>
                                        </div>
                                    </li>
                                </ul>
                            </div>
                            <p v-else class="text-violet-200">Unavailable</p>
                        </div>
                    </div>

                    <span v-else class="text-violet-200 text-sm italic">Off the GitHub Grid</span>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
const route = useRoute()
const isLoading = ref(true)
const scanResult = ref(null)

async function pollForResult(trackingId, maxAttempts = 30, interval = 5000) {
  for (let attempt = 0; attempt < maxAttempts; attempt++) {
    try {
      const statusResult = await $fetch(`/backend/status?id=${trackingId}`, {
        method: 'GET'
      })

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

const expandedDeps = ref([])


function toggleExpand(depName) {
    const index = expandedDeps.value.indexOf(depName)
    if (index > -1) {
        expandedDeps.value.splice(index, 1)
    } else {
        expandedDeps.value.push(depName)
    }
}

function getDependencyData(depName) {
    if (depName in scanResult.value.depends_on) {
        const dependencies = scanResult.value.depends_on[depName];
        const length = dependencies.length;

        if (length > 0) {
            return {
                length: length,
                dependencies: dependencies
            };
        } else {
            return {
                length: 0
            };
        }
    } else {
        return {
            length: 0
        };
    }
}

function cleanDependencyName(dependency) {
    // Split the string at the last '@'
    const lastAtIndex = dependency.lastIndexOf('@');

    if (lastAtIndex <= 0) {
        return dependency;
    }
    return dependency.substring(0, lastAtIndex);
}

function getGitHubURL(dependency) {
    return scanResult.value.objects.find(dep => dep.name === dependency)?.github?.repo_url ?? null;
}

function getFundingInfo(dependency) {
    const fundingInfo = scanResult.value.objects.find(dep => dep.name === dependency)?.github?.funding;
    return fundingInfo?.links || null;
}

function formatPlatformName(platform) {
  if (platform === 'github') return 'GitHub Sponsors';
  
  return platform
    .split(/[-_]/)
    .map(word => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
    .join(' ');
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

<style>
@keyframes fadeIn {
    from {
        opacity: 0;
        transform: translateY(-10px);
    }

    to {
        opacity: 1;
        transform: translateY(0);
    }
}

.animate-fadeIn {
    animation: fadeIn 0.3s ease-out;
}

</style>
