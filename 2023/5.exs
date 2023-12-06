{acc1, acc2} = IO.stream()
|> Enum.reduce(nil, fn
		"seeds:" <> input, nil ->
			seeds = input |> String.split() |> Stream.map(&String.to_integer/1)
			{
				seeds |> Enum.map(&{&1, false}),
				seeds |> Enum.chunk_every(2) |> Enum.map(fn [start, len] -> {start..start+len-1, false} end)
			}

		line, {acc1, acc2} ->
			if line |> String.trim() |> String.ends_with?("map:") do
				{
					acc1 |> Enum.map(fn {curr, _} -> {curr, true} end),
					acc2 |> Enum.map(fn {r, _} -> {r, true} end)
				}
			else
				case line |> String.split() |> Enum.map(&Integer.parse/1) do
					[{dst, ""}, {src, ""}, {len, ""}] ->
						from = src..src+len-1
						offset = dst - src
						{
							acc1 |> Enum.map(fn
									{curr, true} when src <= curr and curr < src + len ->
										{curr + offset, false}
									x -> x
								end),
							acc2 |> Enum.map(fn
									{r, true} ->
										if Range.disjoint?(r, from) do
											[{r, true}]
										else
											IO.puts("r = #{inspect(r)}, from=")
											{bef, r} = Range.split(r, max(0, from.first - r.first))
											{r, aft} = Range.split(r, Range.size(r) - max(0, r.last - from.last))
											t = [{bef, true}, {Range.shift(r, offset), false}, {aft, true}]
											IO.puts(inspect(t))
											t
										end
									x -> [x]
								end)
								|> Enum.reduce(&++/2)
								|> Enum.filter(fn {r, _} -> Range.size(r) > 0 end)
						}
					_ -> {acc1, acc2}
				end
			end
	end)

res1 = acc1 |> Enum.min_by(fn {curr, _} -> curr end) |> elem(0)
IO.puts("Part 1: #{res1}")

IO.puts(inspect(acc2))
res2 = acc2 |> Enum.min_by(fn {r, _} -> r.first end) |> elem(0)
IO.puts("Part 2: #{res2.first}")
