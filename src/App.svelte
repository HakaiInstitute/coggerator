<script lang="ts">
    import Layout from './lib/Layout.svelte'
    import Form from './lib/Form.svelte'
</script>

<main class="h-screen width-full bg-gradient-to-br from-purple-300 via-orange-200 to-cyan-200">
    <Layout>
        <h1 slot="header" class="text-4xl sm:text-6xl text-center text-pink-600 font-sans font-bold flex justify-center">
            <div class="motion-safe:animate-pulse">☁️</div>
            &nbsp;COGGERATOR&nbsp;
            <div class="motion-safe:animate-pulse">☁️</div>
        </h1>
        <Form slot="content"/>
        <p slot="footer" class="text-center text-gray-500 text-xs">
            Made with ❤️ by <a target="_blank" href="https://github.com/tayden">Taylor</a>
        </p>
    </Layout>
</main>
