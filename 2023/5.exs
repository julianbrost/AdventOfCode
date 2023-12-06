"seeds: " <> seedline = IO.read(:line)
seeds = seedline |> String.split() |> Stream.map(&String.to_integer/1)
seeds1 = seeds |> Enum.map(&(&1..&1))
seeds2 = seeds |> Enum.chunk_every(2) |> Enum.map(fn [start, len] -> start..start+len-1 end)

maps = IO.stream()
	|> Enum.reduce([], fn
		line, maps ->
			if line |> String.trim() |> String.ends_with?("map:") do
				[[] | maps]
			else
				case line |> String.split() |> Enum.map(&Integer.parse/1) do
					[{dst, ""}, {src, ""}, {len, ""}] ->
						[[{src..src+len-1, dst - src} | hd maps] | tl maps]
					_ -> maps
				end
			end
		end)
	|> Enum.map(&Enum.reverse/1)
	|> Enum.reverse()

remove_empty = &Enum.filter(&1, fn r -> Range.size(r) > 0 end)

apply_map = fn map, seeds ->
	{mapped, unmapped} = map |> Enum.reduce({[], seeds}, fn
		{src, off}, {mapped, unmapped} -> unmapped |> Enum.reduce({mapped, []}, fn r, {mapped, unmapped} ->
			if Range.disjoint?(r, src) do
				{mapped, [r | unmapped]}
			else
				{bef, r} = Range.split(r, max(0, src.first - r.first))
				{r, aft} = Range.split(r, Range.size(r) - max(0, r.last - src.last))
				{[Range.shift(r, off) | mapped], remove_empty.([bef, aft]) ++ unmapped}
			end
		end)
	end)
	mapped ++ unmapped
end

solve = fn seeds, maps ->
	maps
		|> Enum.reduce(seeds, apply_map)
		|> Enum.map(fn r -> r.first end)
		|> Enum.min()

end

IO.puts("Part 1: #{inspect(solve.(seeds1, maps))}")
IO.puts("Part 2: #{inspect(solve.(seeds2, maps))}")
