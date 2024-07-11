using SharedArrays
using Distributed

y = SharedArray{Float64, 2}(3, 2);

@sync @distributed for i = 1:3
    y[i, :] = rand(2)
end

println(y)