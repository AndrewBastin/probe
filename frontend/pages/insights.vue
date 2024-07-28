<template>
    <Header />
    <LoadingScreen v-if="isLoading" />
    <div v-else-if="scanResult" class="container mx-auto p-4">
        <h1 class="text-3xl font-bold mb-6 text-violet-300">Scan Results</h1>

        <!-- Search, Filter, and Sort Panel -->
        <div class="flex flex-col mb-6 p-4 bg-gray-800 rounded-lg gap-4">
            <div class="flex flex-wrap items-center gap-4">
                <div class="flex-grow">
                    <input v-model="searchQuery" type="text" placeholder="Search dependencies..."
                        class="w-full p-2 bg-gray-700 text-violet-200 rounded">
                </div>
                <div class="flex flex-wrap items-center gap-4">
                    <label class="flex items-center text-violet-200">
                        <input v-model="showWithFundingInfoOnly" type="checkbox" class="mr-2">
                        With Funding Info
                    </label>
                    <label class="flex items-center text-violet-200">
                        <input v-model="showGitHubOnly" type="checkbox" class="mr-2">
                        GitHub Projects
                    </label>
                </div>
            </div>
            <div class="flex items-center">
                <div class="relative inline-flex">
                    <select v-model="sortBy"
                        class="appearance-none bg-gray-700 text-violet-200 pl-3 pr-10 py-2 rounded-l-md border-r border-gray-600">
                        <option value="">Sort by</option>
                        <option value="contributors">Contributors</option>
                        <option value="openIssues">Open Issues</option>
                        <option value="dependencies">Dependencies</option>
                    </select>
                    <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-violet-200">
                        <svg class="fill-current h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20">
                            <path d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z" />
                        </svg>
                    </div>
                </div>
                <button @click="toggleSortOrder" class="bg-gray-700 text-violet-200 px-3 py-2 rounded-r-md">
                    <span v-if="sortOrder === 'asc'" class="block transform rotate-180">▼</span>
                    <span v-else>▼</span>
                </button>
            </div>
        </div>

        <div class="border border-violet-600 rounded-lg overflow-hidden shadow-lg bg-gray-800 bg-opacity-50">
            <div v-for="(dep, _index) in sortedAndFilteredDependencies" :key="dep.name"
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
                                <Icon name="mdi:github" />
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

                        <div class="grid grid-cols-2 gap-4 lg:grid-cols-3">
                            <div class="bg-gray-700 bg-opacity-50 p-4 rounded-lg shadow-inner">
                                <h3 class="font-semibold text-sm text-violet-300 mb-2">Contributors</h3>
                                {{ dep.github?.no_of_contributors || 0 }}
                            </div>
                            <div class="bg-gray-700 bg-opacity-50 p-4 rounded-lg shadow-inner">
                                <h3 class="font-semibold text-sm text-violet-300 mb-2">Open Issues</h3>
                                {{ dep.github?.open_issues || 0 }}
                            </div>
                            <div class="bg-gray-700 bg-opacity-50 p-4 rounded-lg shadow-inner">
                                <h3 class="font-semibold text-sm text-violet-300 mb-2">Dependencies</h3>
                                {{ getDependencyData(dep.name).length }}
                            </div>
                        </div>
                        <div v-if="getDependencyData(dep.name).length > 0"
                            class="bg-gray-700 bg-opacity-50 p-4 rounded-lg shadow-inner">
                            <h3 class="font-semibold text-sm text-violet-300 mb-2">Dependency Info</h3>
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
                            <div v-if="getFundingInfo(dep.name).length > 0">
                                <ul class="list-square marker:text-violet-300 list-outside ml-4">
                                    <li v-for="info in getFundingInfo(dep.name)" :key="info.url" class="mb-4">
                                        <div class="flex flex-col">
                                            <NuxtLink :to="info.url" target="_blank" rel="noopener noreferrer"
                                                class="hover:text-violet-300 transition-colors duration-200 flex items-center mb-2">
                                                {{ formatPlatformName(info.platform) }}
                                                <Icon name="mdi:arrow-top-right" class="ml-1 w-4 h-4" />
                                            </NuxtLink>
                                            <div v-if="Object.keys(info.aiData).length > 0" class="mt-3 space-y-2">
                                                <h4 class="text-sm font-semibold text-violet-300 mb-2">Funding
                                                    Insights</h4>
                                                <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
                                                    <div v-if="info.aiData.monthly_raised_amount"
                                                        class="bg-gray-800 bg-opacity-50 p-3 rounded-lg flex items-center">
                                                        <Icon name="mdi:cash-multiple"
                                                            class="text-green-400 mr-2 w-5 h-5" />
                                                        <div>
                                                            <p class="text-xs text-violet-300">Monthly raised</p>
                                                            <p class="text-sm font-semibold text-violet-100">{{
                                                                info.aiData.monthly_raised_amount }}</p>
                                                        </div>
                                                    </div>
                                                    <div v-if="info.aiData.target_monthly_raised_amount"
                                                        class="bg-gray-800 bg-opacity-50 p-3 rounded-lg flex items-center">
                                                        <Icon name="mdi:target" class="text-blue-400 mr-2 w-5 h-5" />
                                                        <div>
                                                            <p class="text-xs text-violet-300">Monthly target</p>
                                                            <p class="text-sm font-semibold text-violet-100">{{
                                                                info.aiData.target_monthly_raised_amount }}</p>
                                                        </div>
                                                    </div>
                                                    <div v-if="info.aiData.total_raised_amount"
                                                        class="bg-gray-800 bg-opacity-50 p-3 rounded-lg flex items-center">
                                                        <Icon name="mdi:piggy-bank"
                                                            class="text-yellow-400 mr-2 w-5 h-5" />
                                                        <div>
                                                            <p class="text-xs text-violet-300">Total raised</p>
                                                            <p class="text-sm font-semibold text-violet-100">{{
                                                                info.aiData.total_raised_amount }}</p>
                                                        </div>
                                                    </div>
                                                    <div v-if="info.aiData.target_raised_amount"
                                                        class="bg-gray-800 bg-opacity-50 p-3 rounded-lg flex items-center">
                                                        <Icon name="mdi:flag-checkered"
                                                            class="text-red-400 mr-2 w-5 h-5" />
                                                        <div>
                                                            <p class="text-xs text-violet-300">Total target</p>
                                                            <p class="text-sm font-semibold text-violet-100">{{
                                                                info.aiData.target_raised_amount }}</p>
                                                        </div>
                                                    </div>
                                                </div>
                                                <div v-if="info.aiData.is_funding !== undefined"
                                                    class="bg-gray-800 bg-opacity-50 p-3 rounded-lg flex items-center mt-2">
                                                    <Icon
                                                        :name="info.aiData.is_funding ? 'mdi:hand-coin' : 'mdi:hand-coin-off'"
                                                        :class="info.aiData.is_funding ? 'text-green-400' : 'text-red-400'"
                                                        class="mr-2 w-5 h-5" />
                                                    <p class="text-sm text-violet-100">
                                                        {{ info.aiData.is_funding ? `Accepts monetary funding via ${formatPlatformName(info.platform)} ` : `Does not accept monetary funding via ${formatPlatformName(info.platform)}` }}
                                                    </p>
                                                </div>
                                            </div>
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
const expandedDeps = ref([])

// refs for search, filter and sort
const searchQuery = ref('')
const showWithFundingInfoOnly = ref(false)
const showGitHubOnly = ref(false)
const sortBy = ref('')
const sortOrder = ref('asc')

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



const filteredDependencies = computed(() => {
    if (!scanResult.value) return []

    return scanResult.value.objects.filter(dep => {
        const nameMatch = dep.name.toLowerCase().includes(searchQuery.value.toLowerCase())
        const fundingMatch = !showWithFundingInfoOnly.value || (dep.github && dep.github.funding)
        const githubMatch = !showGitHubOnly.value || dep.github

        return nameMatch && fundingMatch && githubMatch
    })
})

const sortedAndFilteredDependencies = computed(() => {
    let deps = [...filteredDependencies.value]

    if (sortBy.value) {
        deps.sort((a, b) => {
            let aValue, bValue;
            switch (sortBy.value) {
                case 'contributors':
                    aValue = a.github?.no_of_contributors || 0
                    bValue = b.github?.no_of_contributors || 0
                    break
                case 'openIssues':
                    aValue = a.github?.open_issues || 0
                    bValue = b.github?.open_issues || 0
                    break
                case 'dependencies':
                    aValue = getDependencyData(a.name).length
                    bValue = getDependencyData(b.name).length
                    break
                default:
                    return 0
            }

            if (sortOrder.value === 'asc') {
                return aValue - bValue
            } else {
                return bValue - aValue
            }
        })
    }
    return deps
})

function toggleSortOrder() {
    sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc'
}
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
    const links = fundingInfo?.links || {};
    const aiInfo = scanResult.value.ai_funding_info || {};

    return Object.entries(links).map(([platform, urls]) => {
        const url = urls[0];
        const aiData = aiInfo[url] || {};
        return {
            platform,
            url,
            aiData
        };
    });
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
