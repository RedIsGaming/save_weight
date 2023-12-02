<script lang="ts">
  import { Icon, CubeTransparent, Home, Bars3 } from "svelte-hero-icons";
</script>

<header class="w-full h-20 bg-transparent border-b-2">
  <nav class="h-20 flex items-center">
    <ul class="w-full ml-10 mr-10 flex items-center justify-between text-white">
      <li>
        <a href="/"><Icon src="{CubeTransparent}" size="50" solid /></a>
      </li>
      <li>
        <a href="/"><Icon src="{Home}" size="50" solid /></a>
      </li>
      <li>
        <a href="/"><Icon src="{Bars3}" size="50" solid /></a>
      </li>
    </ul>
  </nav>
</header>
