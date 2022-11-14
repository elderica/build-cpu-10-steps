module top (
    input wire clk,
    output wire [5:0] led_output
);

    logic [5:0] led = 'd0;
    logic overflow;

    assign led_output = ~led;

    always_ff @ (posedge clk) begin
        if (overflow) begin
            led <= led +'d1;
        end
    end

    timer #(
        .COUNT_MAX (13500000)
    ) inst_0 (
        .clk    (clk),
        .overflow (overflow)
    );
endmodule

`default_nettype wire