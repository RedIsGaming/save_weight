<script lang="ts">
  import { Icon, AtSymbol } from "svelte-hero-icons";

  export function current_year(): number {
    const year: Date = new Date();
    return year.getFullYear();
  }
</script>

<footer class="w-full h-10 bg-transparent bottom-0 left-0 fixed border-t-2">
  <section class="h-10 flex justify-center items-center text-white">
    <Icon src="{AtSymbol}" size="20" solid />
    <p>{current_year()} No Rights Reserved!</p>
  </section>
</footer>
