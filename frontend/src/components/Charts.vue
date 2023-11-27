<script lang="ts">
import { defineComponent, onMounted, onUnmounted, ref } from 'vue';
import { createChart } from 'lightweight-charts';

export default defineComponent({
    setup() {
        const chartRef = ref(null);
        const chart = ref(null);
        let lineSeries = null;
        let dataUpdateInterval = null;

        const fetchHistoricalData = async () => {
            try {
                const historicalData = await fetchHistoricalPrices();
                if (historicalData && lineSeries) {
                    lineSeries.setData(historicalData);
                }
            } catch (error) {
                console.error('Error fetching historical data:', error);
            }
        };

        onMounted(async () => {
            chart.value = createChart(chartRef.value, { width: 800, height: 400 });
            lineSeries = chart.value.addLineSeries();

            // Fetch historical data
            await fetchHistoricalData();
        });

        onUnmounted(() => {
            if (dataUpdateInterval) {
                clearInterval(dataUpdateInterval);
            }
        });

        return {
            chartRef,
        };
    },
});

async function fetchHistoricalPrices() {
    const url = 'https://api.coingecko.com/api/v3/coins/ethereum/market_chart?vs_currency=eur&days=1';
    const response = await fetch(url);
    if (!response.ok) {
        throw new Error('Network response was not ok');
    }
    const data = await response.json();
    const processedData = new Map();

    data.prices.forEach(([timestamp, price]) => {
        const date = new Date(timestamp);
        const formattedDate = date.toISOString().split('T')[0]; // Format: YYYY-MM-DD
        processedData.set(formattedDate, { time: formattedDate, value: price });
    });

    return Array.from(processedData.values()).sort((a, b) => new Date(a.time).getTime() - new Date(b.time).getTime());
}

</script>

<template>
    <div ref="chartRef"></div>
</template>

<style>
    /* Your styles */
</style>
