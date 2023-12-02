<script lang="ts">
  import { Icon, CubeTransparent, Home, Bars3 } from "svelte-hero-icons";
</script>

<header class="w-full h-20 bg-gray-900">
  <nav class="h-20 flex items-center">
    <ul class="w-full ml-10 mr-10 flex items-center justify-between">
      <a href="/"><Icon class="filter invert" src="{CubeTransparent}" size="50" solid /></a>
      <a href="/"><Icon class="filter invert" src="{Home}" size="50" solid /></a>
      
      <a href="/"><Icon class="filter invert" src="{Bars3}" size="50" solid /></a>
    </ul>
  </nav>
</header>
